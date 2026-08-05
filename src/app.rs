// SPDX-License-Identifier: MPL-2.0

use crate::backend::VpnBackend;
use crate::backend::model::{
    ToggleAction, VpnConnection, VpnKind, VpnStatus, format_bytes, format_duration, toggle_action,
};
use crate::backend::nm::NmBackend;
use crate::config::Config;
use crate::fl;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{Alignment, Length, Limits, Subscription, window::Id};
use cosmic::prelude::*;
use cosmic::widget;
use nmrs::agent::{SecretAgentFlags, SecretResponder, SecretSetting};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex as AsyncMutex;

/// A secret value that never reveals its contents when formatted.
#[derive(Clone, Default)]
pub struct Secret(String);

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

/// One secret field to prompt for (e.g. `password`, `otp`).
#[derive(Clone, Debug)]
pub struct SecretField {
    /// NetworkManager secret key.
    pub key: String,
    /// Current entered value (redacted in Debug).
    pub value: Secret,
    /// Whether the value is masked in the input.
    pub hidden: bool,
}

/// Take-once responder wrapper so it can live inside a `Clone + Debug` message.
#[derive(Clone)]
pub struct Responder(Arc<AsyncMutex<Option<SecretResponder>>>);

impl std::fmt::Debug for Responder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<responder>")
    }
}

/// State for an in-progress secret prompt from NetworkManager.
#[derive(Clone, Debug)]
pub struct SecretPrompt {
    /// Display name of the connection.
    pub connection_id: String,
    /// One field per requested secret.
    pub fields: Vec<SecretField>,
    /// Handle to answer NetworkManager.
    pub responder: Responder,
}

/// State of the connection editor (create or edit mode).
#[derive(Default)]
struct Editor {
    /// UUID being edited, or `None` when creating.
    edit_uuid: Option<String>,
    /// Installed VPN plugins to choose from (create mode).
    plugins: Vec<crate::backend::vpn_auth::VpnPlugin>,
    /// Chosen `vpn.service-type`, once a type is picked.
    service_type: Option<String>,
    /// Display name of the chosen type.
    type_name: String,
    /// Connection name.
    name: String,
    /// Whether to auto-activate.
    autoconnect: bool,
    /// Autoconnect priority (text; parsed to i32 on save).
    autoconnect_priority: String,
    /// Metered: 0 automatic, 1 metered, 2 not metered.
    metered: i32,
    /// Firewall zone.
    zone: String,
    /// Split tunnel (never use as default route).
    never_default: bool,
    /// DNS search domains (space separated).
    dns_search: String,
    /// DNS servers (space separated).
    dns: String,
    /// `vpn.data` key/value rows.
    data: Vec<(String, String)>,
    /// True when creating a WireGuard tunnel (uses the WireGuard form below).
    is_wireguard: bool,
    /// WireGuard interface private key.
    wg_private_key: String,
    /// WireGuard interface address (CIDR).
    wg_address: String,
    /// WireGuard DNS servers (space separated).
    wg_dns: String,
    /// WireGuard interface MTU (text; parsed on save).
    wg_mtu: String,
    /// WireGuard peers (at least one).
    wg_peers: Vec<WgPeerForm>,
    /// True while a save is in flight.
    saving: bool,
}

impl Editor {
    /// Current value of a `vpn.data` key (empty string when unset).
    fn data_value(&self, key: &str) -> String {
        self.data
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .unwrap_or_default()
    }

    /// Upsert a `vpn.data` key; an empty value removes it.
    fn set_data(&mut self, key: &str, value: String) {
        self.data.retain(|(k, _)| k != key);
        if !value.is_empty() {
            self.data.push((key.to_string(), value));
        }
    }
}

/// One WireGuard peer row in the editor.
#[derive(Default, Clone)]
struct WgPeerForm {
    /// Peer public key.
    public_key: String,
    /// Peer endpoint (`host:port`).
    endpoint: String,
    /// Peer allowed IPs.
    allowed_ips: String,
    /// Pre-shared key (optional).
    preshared_key: String,
    /// Persistent keepalive seconds (optional).
    persistent_keepalive: String,
}

impl WgPeerForm {
    /// Convert this row into a backend peer spec.
    fn to_spec(&self) -> crate::backend::model::NewWgPeer {
        crate::backend::model::NewWgPeer {
            public_key: self.public_key.trim().to_string(),
            endpoint: self.endpoint.trim().to_string(),
            allowed_ips: self.allowed_ips.clone(),
            preshared_key: self.preshared_key.trim().to_string(),
            persistent_keepalive: self.persistent_keepalive.trim().to_string(),
        }
    }

    /// Build a row from a backend peer spec (prefill).
    fn from_spec(peer: crate::backend::model::NewWgPeer) -> Self {
        Self {
            public_key: peer.public_key,
            endpoint: peer.endpoint,
            allowed_ips: peer.allowed_ips,
            preshared_key: peer.preshared_key,
            persistent_keepalive: peer.persistent_keepalive,
        }
    }
}

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
#[derive(Default)]
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// The popup id.
    popup: Option<Id>,
    /// Configuration data that persists between application runs.
    config: Config,
    /// VPN + WireGuard connections, ordered for display.
    connections: Vec<VpnConnection>,
    /// UUIDs with an operation (activate/deactivate/autoconnect/forget) in flight;
    /// drives the spinner and disables the row until it reconciles.
    pending: HashSet<String>,
    /// UUID of the row whose detail panel is expanded, if any.
    expanded: Option<String>,
    /// When each connection was first observed as Connected, for a live duration.
    connected_since: HashMap<String, Instant>,
    /// Last traffic sample per UUID (rx, tx, when), to compute speed.
    traffic_samples: HashMap<String, (u64, u64, Instant)>,
    /// Latest computed (down/s, up/s) per UUID.
    traffic_speed: HashMap<String, (u64, u64)>,
    /// UUID awaiting a "forget" confirmation, if any.
    confirm_forget: Option<String>,
    /// Active secret prompt from NetworkManager, if any.
    secret_prompt: Option<SecretPrompt>,
    /// The connection editor, when open.
    editor: Option<Editor>,
    /// Set after the first load, so we don't notify for the initial state.
    notifications_ready: bool,
    /// While set and in the future, desktop notifications are muted — used to
    /// swallow the spurious disconnect/reconnect churn around system suspend.
    mute_notifications_until: Option<Instant>,
    /// Last error, if the most recent load or operation failed.
    error: Option<String>,
}

impl AppModel {
    /// Whether desktop notifications are currently muted (e.g. around suspend).
    fn notifications_muted(&self) -> bool {
        self.mute_notifications_until
            .is_some_and(|until| Instant::now() < until)
    }
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    /// Open or close the popup.
    TogglePopup,
    /// The popup with this id was closed.
    PopupClosed(Id),
    /// A refresh succeeded with this connection list.
    Loaded(Vec<VpnConnection>),
    /// A refresh failed with this message.
    LoadFailed(String),
    /// logind `PrepareForSleep`: `true` = suspending, `false` = resuming.
    PrepareForSleep(bool),
    /// Activate the connection with this UUID.
    Activate(String),
    /// Deactivate the connection with this UUID.
    Deactivate(String),
    /// An operation for this UUID finished (activate/deactivate/autoconnect/forget).
    OpFinished {
        /// The affected connection UUID.
        uuid: String,
        /// `Ok` if NetworkManager accepted the request, else the error message.
        result: Result<(), String>,
    },
    /// Expand or collapse the detail panel for this UUID.
    ToggleExpand(String),
    /// Set autoconnect for this UUID.
    SetAutoconnect(String, bool),
    /// Ask to forget this UUID (shows a confirmation).
    RequestForget(String),
    /// Dismiss the forget confirmation.
    CancelForget,
    /// Confirm forgetting this UUID.
    ConfirmForget(String),
    /// Duplicate this connection.
    Duplicate(String),
    /// Periodic tick to refresh the connected-duration display.
    Tick,
    /// NetworkManager requested secrets; show a prompt.
    SecretRequested(SecretPrompt),
    /// The user edited a secret field.
    SecretInput {
        /// Which secret key was edited.
        key: String,
        /// The new value.
        value: Secret,
    },
    /// Toggle masking of a secret field.
    ToggleSecretVisibility(String),
    /// Submit the entered secrets to NetworkManager.
    SubmitSecret,
    /// Dismiss the prompt and tell NetworkManager we won't provide secrets.
    CancelSecret,
    /// NetworkManager cancelled the outstanding secret request.
    SecretCancelled,
    /// A secret response finished sending (no-op).
    SecretDone,
    /// Open the file picker to import a VPN config.
    ImportRequested,
    /// A file was chosen to import.
    ImportPicked(PathBuf),
    /// The file picker was dismissed without a selection.
    ImportDismissed,
    /// An import finished (`Ok`, or the error message).
    ImportFinished(Result<(), String>),
    /// Open the editor to create a new VPN.
    OpenCreate,
    /// Open the editor to edit an existing connection (by UUID).
    OpenEdit(String),
    /// The connection to edit was loaded (UUID + fields, or error).
    EditLoaded {
        /// UUID being edited.
        uuid: String,
        /// Loaded editable fields, or the error message.
        result: Result<crate::backend::model::VpnEdit, String>,
    },
    /// A VPN type was chosen (service-type, display name).
    EditorSelectType(String, String),
    /// The WireGuard type was chosen.
    EditorSelectWireGuard,
    /// Editor: WireGuard private key changed (redacted in Debug).
    EditorWgPrivateKey(Secret),
    /// Editor: WireGuard address changed.
    EditorWgAddress(String),
    /// Editor: WireGuard DNS servers changed.
    EditorWgDns(String),
    /// Editor: WireGuard MTU changed.
    EditorWgMtu(String),
    /// Editor: add an empty WireGuard peer row.
    EditorWgAddPeer,
    /// Editor: remove WireGuard peer at index.
    EditorWgRemovePeer(usize),
    /// Editor: peer `index` public key changed.
    EditorWgPeerKey(usize, String),
    /// Editor: peer `index` endpoint changed.
    EditorWgPeerEndpoint(usize, String),
    /// Editor: peer `index` allowed IPs changed.
    EditorWgAllowedIps(usize, String),
    /// Editor: peer `index` pre-shared key changed (redacted in Debug).
    EditorWgPeerPsk(usize, Secret),
    /// Editor: peer `index` persistent keepalive changed.
    EditorWgPeerKeepalive(usize, String),
    /// A saved WireGuard tunnel's editable fields were loaded (or failed).
    WireGuardEditLoaded {
        /// UUID being edited.
        uuid: String,
        /// Loaded editable fields, or the error message.
        result: Result<crate::backend::model::WgEdit, String>,
    },
    /// Editor: name changed.
    EditorName(String),
    /// Editor: autoconnect toggled.
    EditorAutoconnect(bool),
    /// Editor: autoconnect priority changed.
    EditorPriority(String),
    /// Editor: metered mode changed.
    EditorMetered(i32),
    /// Editor: firewall zone changed.
    EditorZone(String),
    /// Editor: split-tunnel toggled.
    EditorNeverDefault(bool),
    /// Editor: DNS search domains changed.
    EditorDnsSearch(String),
    /// Editor: DNS servers changed.
    EditorDns(String),
    /// Editor: a data row's key changed.
    EditorDataKey(usize, String),
    /// Editor: a data row's value changed.
    EditorDataValue(usize, String),
    /// Editor: browse for a file to put in a data row's value.
    EditorBrowseData(usize),
    /// Editor: a file was chosen for a data row.
    EditorDataPicked(usize, String),
    /// Editor: set a `vpn.data` key by name (structured per-protocol forms).
    EditorDataSet(String, String),
    /// Editor: browse for a file to put in a named `vpn.data` key.
    EditorBrowseDataKey(String),
    /// Editor: a file was chosen for a named `vpn.data` key.
    EditorDataKeyPicked(String, String),
    /// Editor: add a data row.
    EditorAddRow,
    /// Editor: remove a data row.
    EditorRemoveRow(usize),
    /// Editor: save the new connection.
    EditorSave,
    /// Editor: cancel and close.
    EditorCancel,
    /// A create finished (`Ok`, or the error message).
    EditorSaved(Result<(), String>),
    /// A VPN failed to activate (name + reason code).
    VpnFailed {
        /// Connection name.
        name: String,
        /// NM failure reason code.
        reason: u32,
    },
    /// Desktop notifications finished sending (no-op).
    NotificationsSent,
    /// The application configuration changed on disk.
    UpdateConfig(Config),
}

/// Load the connection list and turn the result into a message.
async fn load_message() -> Message {
    match NmBackend::new().list_connections().await {
        Ok(connections) => Message::Loaded(connections),
        Err(error) => Message::LoadFailed(error.to_string()),
    }
}

/// Spawn a one-shot background task that refreshes the connection list.
fn load_task() -> Task<cosmic::Action<Message>> {
    cosmic::task::future(load_message()).map(cosmic::Action::App)
}

/// Send a desktop notification via the `org.freedesktop.Notifications` service.
async fn send_notification(summary: String, body: String) {
    let Ok(connection) = zbus::Connection::session().await else {
        return;
    };
    let Ok(proxy) = zbus::Proxy::new(
        &connection,
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
    )
    .await
    else {
        return;
    };
    let hints: HashMap<&str, zbus::zvariant::Value<'_>> = HashMap::new();
    let _: Result<u32, _> = proxy
        .call(
            "Notify",
            &(
                "COSMIC VPN Menu",
                0u32,
                "network-vpn-symbolic",
                summary,
                body,
                Vec::<&str>::new(),
                hints,
                -1i32,
            ),
        )
        .await;
}

/// Spawn a task that sends a batch of `(summary, body)` notifications.
fn notify_task(events: Vec<(String, String)>) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        for (summary, body) in events {
            send_notification(summary, body).await;
        }
        Message::NotificationsSent
    })
    .map(cosmic::Action::App)
}

/// Compute the notifications to send for status transitions between the old and new
/// connection lists (connected / disconnected).
fn status_transitions(old: &[VpnConnection], new: &[VpnConnection]) -> Vec<(String, String)> {
    let previous: HashMap<&str, VpnStatus> =
        old.iter().map(|c| (c.uuid.as_str(), c.status)).collect();
    let mut events = Vec::new();
    for connection in new {
        let was = previous.get(connection.uuid.as_str()).copied();
        let Some(was) = was else { continue };
        match connection.status {
            VpnStatus::Connected if was != VpnStatus::Connected => {
                events.push((fl!("notify-connected"), connection.id.clone()));
            }
            VpnStatus::Disconnected
                if matches!(
                    was,
                    VpnStatus::Connected | VpnStatus::Connecting | VpnStatus::Disconnecting
                ) =>
            {
                events.push((fl!("notify-disconnected"), connection.id.clone()));
            }
            _ => {}
        }
    }
    events
}

/// Open the XDG portal file picker for a `.ovpn` / WireGuard `.conf`.
fn pick_import_file() -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async {
        use cosmic::dialog::file_chooser;
        let filter = file_chooser::FileFilter::new("VPN configuration")
            .glob("*.ovpn")
            .glob("*.conf");
        match file_chooser::open::Dialog::new()
            .filter(filter)
            .open_file()
            .await
        {
            Ok(response) => match response.url().to_file_path() {
                Ok(path) => Message::ImportPicked(path),
                Err(()) => Message::ImportFinished(Err("selected file is not local".to_string())),
            },
            Err(file_chooser::Error::Cancelled) => Message::ImportDismissed,
            Err(error) => Message::ImportFinished(Err(error.to_string())),
        }
    })
    .map(cosmic::Action::App)
}

/// Open the file picker to choose a file for editor data row `index`.
fn pick_data_file(index: usize) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        use cosmic::dialog::file_chooser;
        match file_chooser::open::Dialog::new().open_file().await {
            Ok(response) => match response.url().to_file_path() {
                Ok(path) => Message::EditorDataPicked(index, path.display().to_string()),
                Err(()) => Message::ImportDismissed,
            },
            Err(_) => Message::ImportDismissed,
        }
    })
    .map(cosmic::Action::App)
}

/// A labelled text field bound to a named `vpn.data` key.
fn data_text_field(label: String, key: &'static str, value: String) -> Element<'static, Message> {
    widget::settings::item(
        label,
        widget::text_input("", value)
            .on_input(move |v| Message::EditorDataSet(key.to_string(), v))
            .width(Length::Fixed(260.0)),
    )
    .into()
}

/// A labelled file field (text + browse button) bound to a named `vpn.data` key.
fn data_file_field(label: String, key: &'static str, value: String) -> Element<'static, Message> {
    widget::settings::item(
        label,
        widget::Row::with_children(vec![
            widget::text_input("", value)
                .on_input(move |v| Message::EditorDataSet(key.to_string(), v))
                .width(Length::Fixed(220.0))
                .into(),
            cosmic::applet::menu_button(
                widget::icon::from_name("document-open-symbolic")
                    .size(16)
                    .symbolic(true),
            )
            .width(Length::Shrink)
            .on_press(Message::EditorBrowseDataKey(key.to_string()))
            .into(),
        ])
        .spacing(4.0)
        .align_y(Alignment::Center),
    )
    .into()
}

/// A labelled toggle bound to a `yes`/unset `vpn.data` key.
fn data_toggle_field(label: String, key: &'static str, on: bool) -> Element<'static, Message> {
    widget::settings::item(
        label,
        widget::toggler(on).on_toggle(move |v| {
            let value = if v { "yes".to_string() } else { String::new() };
            Message::EditorDataSet(key.to_string(), value)
        }),
    )
    .into()
}

/// A labelled dropdown mapping display `labels` to `values` for a `vpn.data` key.
fn data_choice_field(
    label: String,
    key: &'static str,
    values: &'static [&'static str],
    labels: Vec<String>,
    current: String,
) -> Element<'static, Message> {
    let selected = values.iter().position(|v| *v == current).or(Some(0));
    widget::settings::item(
        label,
        widget::dropdown(labels, selected, move |i| {
            Message::EditorDataSet(key.to_string(), values[i].to_string())
        })
        .width(Length::Fixed(260.0)),
    )
    .into()
}

/// A dropdown for a closed-enum `vpn.data` key (values from [`crate::backend::enums`]),
/// with a leading "Default" (unset) choice.
fn data_enum_field(
    label: String,
    key: &'static str,
    service_type: &str,
    current: String,
) -> Element<'static, Message> {
    let options = crate::backend::enums::enum_values(service_type, key).unwrap_or(&[]);
    let mut values: Vec<String> = vec![String::new()];
    values.extend(options.iter().map(|s| (*s).to_string()));
    let mut labels: Vec<String> = vec![fl!("oc-os-default")];
    labels.extend(options.iter().map(|s| (*s).to_string()));
    let selected = values.iter().position(|v| *v == current).or(Some(0));
    widget::settings::item(
        label,
        widget::dropdown(labels, selected, move |i| {
            Message::EditorDataSet(key.to_string(), values[i].clone())
        })
        .width(Length::Fixed(260.0)),
    )
    .into()
}

/// Open the file picker to choose a file for the named `vpn.data` `key`.
fn pick_data_file_key(key: String) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        use cosmic::dialog::file_chooser;
        match file_chooser::open::Dialog::new().open_file().await {
            Ok(response) => match response.url().to_file_path() {
                Ok(path) => Message::EditorDataKeyPicked(key, path.display().to_string()),
                Err(()) => Message::ImportDismissed,
            },
            Err(_) => Message::ImportDismissed,
        }
    })
    .map(cosmic::Action::App)
}

/// Import the chosen config file in the background.
fn import_task(path: PathBuf) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .import(&path)
            .await
            .map_err(|e| e.to_string());
        Message::ImportFinished(result)
    })
    .map(cosmic::Action::App)
}

/// Create a new VPN connection in the background.
fn create_task(spec: crate::backend::model::NewVpn) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .create_vpn(spec)
            .await
            .map_err(|e| e.to_string());
        Message::EditorSaved(result)
    })
    .map(cosmic::Action::App)
}

/// Create a new WireGuard connection in the background.
fn create_wireguard_task(
    spec: crate::backend::model::NewWireGuard,
) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .create_wireguard(spec)
            .await
            .map_err(|e| e.to_string());
        Message::EditorSaved(result)
    })
    .map(cosmic::Action::App)
}

/// Load an existing connection's editable fields.
fn read_task(uuid: String) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .read_vpn(&uuid)
            .await
            .map_err(|e| e.to_string());
        Message::EditLoaded { uuid, result }
    })
    .map(cosmic::Action::App)
}

/// Load an existing WireGuard tunnel's editable fields.
fn read_wireguard_task(uuid: String) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .read_wireguard(&uuid)
            .await
            .map_err(|e| e.to_string());
        Message::WireGuardEditLoaded { uuid, result }
    })
    .map(cosmic::Action::App)
}

/// Update an existing WireGuard tunnel in the background.
fn update_wireguard_task(
    uuid: String,
    spec: crate::backend::model::NewWireGuard,
) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .update_wireguard(&uuid, spec)
            .await
            .map_err(|e| e.to_string());
        Message::EditorSaved(result)
    })
    .map(cosmic::Action::App)
}

/// Duplicate a connection in the background.
fn duplicate_task(uuid: String) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .duplicate(&uuid)
            .await
            .map_err(|e| e.to_string());
        Message::EditorSaved(result)
    })
    .map(cosmic::Action::App)
}

/// Update an existing VPN connection in the background.
fn update_task(uuid: String, spec: crate::backend::model::NewVpn) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let result = NmBackend::new()
            .update_vpn(&uuid, spec)
            .await
            .map_err(|e| e.to_string());
        Message::EditorSaved(result)
    })
    .map(cosmic::Action::App)
}

/// Long-lived subscription: reload on every NetworkManager change event, and survive
/// NM restart / D-Bus loss by reconnecting after a short delay (no polling).
fn nm_subscription() -> Subscription<Message> {
    Subscription::run(|| {
        cosmic::iced::stream::channel(
            16,
            |mut output: cosmic::iced::futures::channel::mpsc::Sender<Message>| async move {
                use cosmic::iced::futures::{SinkExt, StreamExt};

                loop {
                    // Emit current state on first connect and after every reconnect.
                    let _ = output.send(load_message().await).await;

                    match crate::backend::nm::event_ticks().await {
                        Ok(mut ticks) => {
                            while ticks.next().await.is_some() {
                                let _ = output.send(load_message().await).await;
                            }
                            // Stream ended: NetworkManager went away.
                        }
                        Err(error) => {
                            let _ = output.send(Message::LoadFailed(error.to_string())).await;
                        }
                    }

                    // Back off before reconnecting so we don't spin while NM is down.
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                }
            },
        )
    })
}

/// UUIDs whose auth-dialog helper is currently running, so a repeat GetSecrets doesn't
/// launch a second dialog.
type InFlight = Arc<std::sync::Mutex<HashSet<String>>>;

/// Long-lived subscription: register as an NM secret agent and handle VPN secret
/// requests, reconnecting on NM restart. OpenConnect (and other auth-dialog service
/// types) are routed to the plugin helper; other interactive VPN requests show a
/// native prompt; non-VPN / non-interactive requests are declined.
fn secret_agent_subscription() -> Subscription<Message> {
    Subscription::run(|| {
        cosmic::iced::stream::channel(
            16,
            |mut output: cosmic::iced::futures::channel::mpsc::Sender<Message>| async move {
                use cosmic::iced::futures::{SinkExt, StreamExt};

                let in_flight: InFlight = Arc::new(std::sync::Mutex::new(HashSet::new()));

                loop {
                    match crate::backend::nm::register_secret_agent().await {
                        Ok((mut handle, mut requests)) => loop {
                            tokio::select! {
                                request = requests.next() => match request {
                                    Some(request) => {
                                        if let Some(message) =
                                            handle_secret_request(request, &in_flight).await
                                        {
                                            let _ = output.send(message).await;
                                        }
                                    }
                                    None => break, // NetworkManager went away.
                                },
                                cancel = handle.cancellations().next() => {
                                    if cancel.is_some() {
                                        let _ = output.send(Message::SecretCancelled).await;
                                    }
                                }
                            }
                        },
                        Err(error) => {
                            let _ = output.send(Message::LoadFailed(error.to_string())).await;
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            },
        )
    })
}

/// Route an NM secret request. Returns a prompt message for the native password path,
/// or `None` when the request is declined or handled by the auth-dialog helper.
async fn handle_secret_request(
    request: nmrs::agent::SecretRequest,
    in_flight: &InFlight,
) -> Option<Message> {
    let interactive = request.flags.contains(SecretAgentFlags::ALLOW_INTERACTION);

    let SecretSetting::Vpn { service_type, .. } = &request.setting else {
        // Not a VPN request; decline so other agents can handle it.
        let _ = request.responder.no_secrets().await;
        return None;
    };
    let service_type = service_type.clone();

    if !interactive {
        let _ = request.responder.no_secrets().await;
        return None;
    }

    // OpenConnect etc.: let the plugin's auth-dialog own the (possibly SAML) login.
    if crate::backend::vpn_auth::uses_auth_dialog(&service_type) {
        spawn_auth_dialog_flow(request, service_type, in_flight.clone());
        return None;
    }

    // Native password/OTP prompt.
    let keys = if request.hints.is_empty() {
        vec!["password".to_string()]
    } else {
        request.hints.clone()
    };
    let fields = keys
        .into_iter()
        .map(|key| SecretField {
            key,
            value: Secret::default(),
            hidden: true,
        })
        .collect();
    Some(Message::SecretRequested(SecretPrompt {
        connection_id: request.connection_id,
        fields,
        responder: Responder(Arc::new(AsyncMutex::new(Some(request.responder)))),
    }))
}

/// Spawn the auth-dialog flow for a request (unless one is already running for its
/// UUID), answering NetworkManager when the helper returns.
fn spawn_auth_dialog_flow(
    request: nmrs::agent::SecretRequest,
    service_type: String,
    in_flight: InFlight,
) {
    let uuid = request.connection_uuid.clone();
    if !in_flight.lock().unwrap().insert(uuid.clone()) {
        // A dialog is already running for this connection; decline the duplicate.
        tokio::spawn(async move {
            let _ = request.responder.no_secrets().await;
        });
        return;
    }

    tokio::spawn(async move {
        match run_openconnect_auth(&request, &service_type).await {
            Ok(secrets) => {
                let _ = request.responder.vpn_secrets(secrets).await;
            }
            Err(_) => {
                let _ = request.responder.cancel().await;
            }
        }
        in_flight.lock().unwrap().remove(&uuid);
    });
}

/// Drive the auth-dialog helper for one request and return the secrets it produces.
async fn run_openconnect_auth(
    request: &nmrs::agent::SecretRequest,
    service_type: &str,
) -> Result<HashMap<String, String>, crate::backend::BackendError> {
    use crate::backend::vpn_auth;

    let auth_dialog = vpn_auth::find_auth_dialog(service_type).ok_or_else(|| {
        crate::backend::BackendError(format!("no auth-dialog found for {service_type}"))
    })?;
    let allow_interaction = request.flags.contains(SecretAgentFlags::ALLOW_INTERACTION);
    let request_new = request.flags.contains(SecretAgentFlags::REQUEST_NEW);
    let argv = vpn_auth::build_argv(
        &request.connection_uuid,
        &request.connection_id,
        service_type,
        allow_interaction,
        request_new,
    );
    let data = vpn_auth::fetch_vpn_data(request.connection_path.as_str()).await?;
    let existing: Vec<(String, String)> = request
        .existing_secrets
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    let payload = vpn_auth::serialize_stdin(&data, &existing);
    vpn_auth::run_auth_dialog(&auth_dialog, &argv, &payload).await
}

/// Long-lived subscription: surface VPN activation failures (with their reason),
/// reconnecting if the bus drops.
fn vpn_failure_subscription() -> Subscription<Message> {
    Subscription::run(|| {
        cosmic::iced::stream::channel(
            16,
            |mut output: cosmic::iced::futures::channel::mpsc::Sender<Message>| async move {
                use cosmic::iced::futures::{SinkExt, StreamExt};
                loop {
                    if let Ok(mut failures) = crate::backend::nm::vpn_failure_events().await {
                        while let Some(failure) = failures.next().await {
                            let _ = output
                                .send(Message::VpnFailed {
                                    name: failure.name,
                                    reason: failure.reason,
                                })
                                .await;
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            },
        )
    })
}

/// Watch logind for suspend/resume so we can mute notifications across a sleep.
fn sleep_subscription() -> Subscription<Message> {
    Subscription::run(|| {
        cosmic::iced::stream::channel(
            4,
            |mut output: cosmic::iced::futures::channel::mpsc::Sender<Message>| async move {
                use cosmic::iced::futures::{SinkExt, StreamExt};
                loop {
                    if let Ok(mut events) = crate::backend::nm::sleep_events().await {
                        while let Some(going_to_sleep) = events.next().await {
                            let _ = output.send(Message::PrepareForSleep(going_to_sleep)).await;
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            },
        )
    })
}

/// Localized message for a VPN failure reason code, or `None` for non-actionable
/// reasons (unknown / user-initiated).
fn reason_text(reason: u32) -> Option<String> {
    Some(match reason {
        5 => fl!("reason-ip-config"),
        6 | 7 => fl!("reason-timeout"),
        8 => fl!("reason-service-failed"),
        9 => fl!("reason-no-secrets"),
        10 => fl!("reason-login-failed"),
        _ => return None,
    })
}

/// Panel icon name reflecting the aggregate connection state.
fn panel_icon(connections: &[VpnConnection]) -> &'static str {
    match crate::backend::model::overall_status(connections) {
        VpnStatus::Connected => "network-vpn-symbolic",
        VpnStatus::Disconnected => "network-vpn-disconnected-symbolic",
        // Connecting / Disconnecting / Unknown: transitioning.
        _ => "network-vpn-acquiring-symbolic",
    }
}

/// Turn a backend operation result into an `OpFinished` message for `uuid`.
fn op_finished(uuid: String, outcome: Result<(), crate::backend::BackendError>) -> Message {
    Message::OpFinished {
        uuid,
        result: outcome.map_err(|error| error.to_string()),
    }
}

/// Spawn a background task that activates (`up == true`) or deactivates a connection.
fn op_task(uuid: String, up: bool) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let backend = NmBackend::new();
        let outcome = if up {
            backend.activate(&uuid).await
        } else {
            backend.deactivate(&uuid).await
        };
        op_finished(uuid, outcome)
    })
    .map(cosmic::Action::App)
}

/// Spawn a background task that sets autoconnect for a connection.
fn autoconnect_task(uuid: String, on: bool) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let outcome = NmBackend::new().set_autoconnect(&uuid, on).await;
        op_finished(uuid, outcome)
    })
    .map(cosmic::Action::App)
}

/// Spawn a background task that forgets (deletes) a connection.
fn forget_task(uuid: String) -> Task<cosmic::Action<Message>> {
    cosmic::task::future(async move {
        let outcome = NmBackend::new().forget(&uuid).await;
        op_finished(uuid, outcome)
    })
    .map(cosmic::Action::App)
}

/// Localized label for a status.
fn status_label(status: VpnStatus) -> String {
    match status {
        VpnStatus::Connected => fl!("vpn-status-connected"),
        VpnStatus::Connecting => fl!("vpn-status-connecting"),
        VpnStatus::Disconnecting => fl!("vpn-status-disconnecting"),
        VpnStatus::Disconnected => fl!("vpn-status-disconnected"),
        VpnStatus::Unknown => fl!("vpn-status-unknown"),
    }
}

/// A `label: value` detail line.
fn detail_row<'a>(label: String, value: String) -> Element<'a, Message> {
    let spacing = cosmic::theme::active().cosmic().spacing;
    widget::Row::with_children(vec![
        widget::text(format!("{label}:")).into(),
        widget::text(value).width(Length::Fill).into(),
    ])
    .spacing(spacing.space_s)
    .into()
}

impl AppModel {
    /// Open the popup if it isn't already open.
    fn open_popup(&mut self) -> Task<cosmic::Action<Message>> {
        if self.popup.is_some() {
            return Task::none();
        }
        let new_id = Id::unique();
        self.popup.replace(new_id);
        let mut popup_settings = self.core.applet.get_popup_settings(
            self.core.main_window_id().unwrap(),
            new_id,
            None,
            None,
            None,
        );
        popup_settings.positioner.size_limits = Limits::NONE
            .max_width(372.0)
            .min_width(300.0)
            .min_height(200.0)
            .max_height(1080.0);
        get_popup(popup_settings)
    }

    /// Optimistically set the status of one connection (by UUID), if present.
    fn set_status(&mut self, uuid: &str, status: VpnStatus) {
        if let Some(connection) = self.connections.iter_mut().find(|c| c.uuid == uuid) {
            connection.status = status;
        }
    }

    /// Update traffic samples and per-second speeds from a fresh connection list.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn update_traffic(&mut self, connections: &[VpnConnection]) {
        let now = Instant::now();
        for connection in connections {
            if connection.status != VpnStatus::Connected {
                continue;
            }
            let key = connection.uuid.clone();
            if let Some((prev_rx, prev_tx, at)) = self.traffic_samples.get(&key).copied() {
                let elapsed = now.duration_since(at).as_secs_f64();
                if elapsed >= 0.5 {
                    let down =
                        (connection.rx_bytes.saturating_sub(prev_rx) as f64 / elapsed) as u64;
                    let up = (connection.tx_bytes.saturating_sub(prev_tx) as f64 / elapsed) as u64;
                    self.traffic_speed.insert(key.clone(), (down, up));
                    self.traffic_samples
                        .insert(key, (connection.rx_bytes, connection.tx_bytes, now));
                }
            } else {
                self.traffic_samples
                    .insert(key, (connection.rx_bytes, connection.tx_bytes, now));
            }
        }
        let connected: HashSet<&str> = connections
            .iter()
            .filter(|c| c.status == VpnStatus::Connected)
            .map(|c| c.uuid.as_str())
            .collect();
        self.traffic_samples
            .retain(|uuid, _| connected.contains(uuid.as_str()));
        self.traffic_speed
            .retain(|uuid, _| connected.contains(uuid.as_str()));
    }

    /// Stamp newly-connected connections and drop stamps for ones no longer up, so
    /// the detail panel can show a session-local "connected for" duration.
    fn update_durations(&mut self, connections: &[VpnConnection]) {
        let now = Instant::now();
        let connected: HashSet<&str> = connections
            .iter()
            .filter(|c| c.status == VpnStatus::Connected)
            .map(|c| c.uuid.as_str())
            .collect();
        for uuid in &connected {
            self.connected_since
                .entry((*uuid).to_string())
                .or_insert(now);
        }
        self.connected_since
            .retain(|uuid, _| connected.contains(uuid.as_str()));
    }

    /// Render one connection: a header (connect button + expand chevron) and, when
    /// expanded, a detail panel.
    fn connection_view(&self, connection: &VpnConnection) -> Element<'_, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let uuid = connection.uuid.clone();
        let pending = self.pending.contains(&uuid);
        let expanded = self.expanded.as_deref() == Some(uuid.as_str());

        let trailing: Element<'_, Message> = if pending {
            widget::indeterminate_circular().size(20.0).into()
        } else {
            widget::text(status_label(connection.status)).into()
        };

        let connect_content = widget::Row::with_children(vec![
            widget::icon::from_name("network-vpn-symbolic")
                .size(16)
                .symbolic(true)
                .into(),
            widget::text(connection.id.clone())
                .width(Length::Fill)
                .into(),
            trailing,
        ])
        .align_y(Alignment::Center)
        .spacing(spacing.space_s);

        let mut connect_button = cosmic::applet::menu_button(connect_content).width(Length::Fill);
        if !pending {
            connect_button = connect_button.on_press(match toggle_action(connection.status) {
                ToggleAction::Deactivate => Message::Deactivate(uuid.clone()),
                ToggleAction::Activate => Message::Activate(uuid.clone()),
            });
        }

        let chevron_icon = if expanded {
            "go-up-symbolic"
        } else {
            "go-down-symbolic"
        };
        let chevron = cosmic::applet::menu_button(
            widget::icon::from_name(chevron_icon)
                .size(16)
                .symbolic(true),
        )
        .width(Length::Shrink)
        .on_press(Message::ToggleExpand(uuid.clone()));

        let header = widget::Row::with_children(vec![connect_button.into(), chevron.into()])
            .align_y(Alignment::Center);

        let mut column = widget::Column::new().push(header);
        if expanded {
            column = column.push(self.detail_view(connection));
        }
        column.into()
    }

    /// The expanded detail/management panel for one connection.
    fn detail_view(&self, connection: &VpnConnection) -> Element<'_, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let uuid = connection.uuid.clone();

        let mut items = widget::Column::new()
            .spacing(spacing.space_xxs)
            .padding([spacing.space_xxs, spacing.space_m]);

        items = items.push(detail_row(
            fl!("detail-type"),
            connection.type_label.clone(),
        ));
        if let Some(gateway) = &connection.gateway {
            items = items.push(detail_row(fl!("detail-gateway"), gateway.clone()));
        }
        if let Some(ip4) = &connection.ip4 {
            items = items.push(detail_row(fl!("detail-ipv4"), ip4.clone()));
        }
        if let Some(ip6) = &connection.ip6 {
            items = items.push(detail_row(fl!("detail-ipv6"), ip6.clone()));
        }
        if !connection.dns_servers.is_empty() {
            items = items.push(detail_row(
                fl!("detail-dns"),
                connection.dns_servers.join(", "),
            ));
        }
        if connection.status == VpnStatus::Connected
            && (connection.rx_bytes > 0 || connection.tx_bytes > 0)
        {
            let (down, up) = self.traffic_speed.get(&uuid).copied().unwrap_or((0, 0));
            items = items.push(detail_row(
                fl!("detail-received"),
                format!(
                    "{} ({}/s)",
                    format_bytes(connection.rx_bytes),
                    format_bytes(down)
                ),
            ));
            items = items.push(detail_row(
                fl!("detail-sent"),
                format!(
                    "{} ({}/s)",
                    format_bytes(connection.tx_bytes),
                    format_bytes(up)
                ),
            ));
        }
        if let Some(since) = self.connected_since.get(&uuid) {
            items = items.push(detail_row(
                fl!("detail-duration"),
                format_duration(since.elapsed().as_secs()),
            ));
        } else {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            if let Some(last) = crate::backend::model::format_last_used(connection.last_used, now) {
                items = items.push(detail_row(fl!("detail-last-used"), last));
            }
        }

        // Autoconnect toggle.
        let uuid_for_toggle = uuid.clone();
        items = items.push(widget::settings::item(
            fl!("autoconnect"),
            widget::toggler(connection.autoconnect)
                .on_toggle(move |on| Message::SetAutoconnect(uuid_for_toggle.clone(), on)),
        ));

        // Edit (plugin VPNs and WireGuard tunnels).
        items = items.push(
            cosmic::applet::menu_button(
                widget::Row::with_children(vec![
                    widget::icon::from_name("document-edit-symbolic")
                        .size(16)
                        .symbolic(true)
                        .into(),
                    widget::text(fl!("edit")).into(),
                ])
                .align_y(Alignment::Center)
                .spacing(spacing.space_s),
            )
            .on_press(Message::OpenEdit(uuid.clone())),
        );

        // Duplicate (any type).
        items = items.push(
            cosmic::applet::menu_button(
                widget::Row::with_children(vec![
                    widget::icon::from_name("edit-copy-symbolic")
                        .size(16)
                        .symbolic(true)
                        .into(),
                    widget::text(fl!("duplicate")).into(),
                ])
                .align_y(Alignment::Center)
                .spacing(spacing.space_s),
            )
            .on_press(Message::Duplicate(uuid.clone())),
        );

        // Forget, with a confirmation step.
        let forget: Element<'_, Message> = if self.confirm_forget.as_deref() == Some(uuid.as_str())
        {
            let delete = cosmic::applet::menu_button(widget::text(fl!("delete")))
                .width(Length::Fill)
                .on_press(Message::ConfirmForget(uuid.clone()));
            let cancel = cosmic::applet::menu_button(widget::text(fl!("cancel")))
                .width(Length::Fill)
                .on_press(Message::CancelForget);
            widget::Column::new()
                .spacing(spacing.space_xxs)
                .push(widget::text(fl!("confirm-forget")))
                .push(
                    widget::Row::with_children(vec![delete.into(), cancel.into()])
                        .spacing(spacing.space_xs),
                )
                .into()
        } else {
            cosmic::applet::menu_button(
                widget::Row::with_children(vec![
                    widget::icon::from_name("user-trash-symbolic")
                        .size(16)
                        .symbolic(true)
                        .into(),
                    widget::text(fl!("forget")).into(),
                ])
                .align_y(Alignment::Center)
                .spacing(spacing.space_s),
            )
            .on_press(Message::RequestForget(uuid.clone()))
            .into()
        };
        items = items.push(forget);

        items.into()
    }

    /// The password/OTP prompt shown when NetworkManager requests secrets.
    fn secret_prompt_view(&self, prompt: &SecretPrompt) -> Element<'_, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        let mut column = widget::Column::new()
            .spacing(spacing.space_xxs)
            .push(widget::text(fl!(
                "secret-prompt",
                name = prompt.connection_id.clone()
            )));

        for field in &prompt.fields {
            let key = field.key.clone();
            let input_key = key.clone();
            let input = widget::secure_input(
                field.key.clone(),
                field.value.0.clone(),
                Some(Message::ToggleSecretVisibility(key)),
                field.hidden,
            )
            .on_input(move |value| Message::SecretInput {
                key: input_key.clone(),
                value: Secret(value),
            })
            .on_submit(|_| Message::SubmitSecret)
            .width(Length::Fill);
            column = column.push(input);
        }

        let buttons = widget::Row::with_children(vec![
            widget::button::standard(fl!("cancel"))
                .on_press(Message::CancelSecret)
                .into(),
            widget::button::suggested(fl!("connect"))
                .on_press(Message::SubmitSecret)
                .into(),
        ])
        .spacing(spacing.space_xs);

        column.push(buttons).into()
    }

    /// The connection editor (create mode): pick a type, then fill in the form.
    fn editor_view(&self, editor: &Editor) -> Element<'_, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let mut column = widget::Column::new()
            .spacing(spacing.space_xs)
            .padding([spacing.space_xxs, spacing.space_m]);

        // Step 1: choose the VPN type.
        let Some(_service_type) = &editor.service_type else {
            column = column.push(widget::text(fl!("select-vpn-type")));
            // WireGuard is a native device type, not a plugin, so list it explicitly.
            column = column.push(
                cosmic::applet::menu_button(widget::text("WireGuard"))
                    .on_press(Message::EditorSelectWireGuard),
            );
            for plugin in &editor.plugins {
                let service_type = plugin.service_type.clone();
                let name = plugin.name.clone();
                column = column.push(
                    cosmic::applet::menu_button(widget::text(plugin.name.clone()))
                        .on_press(Message::EditorSelectType(service_type, name)),
                );
            }
            column = column
                .push(widget::button::standard(fl!("cancel")).on_press(Message::EditorCancel));
            return widget::scrollable(column).into();
        };

        // WireGuard uses a bespoke form (interface + one or more peers).
        if editor.is_wireguard {
            let field = |label: String, value: String, on_input: fn(String) -> Message| {
                widget::Column::new()
                    .spacing(spacing.space_xxs)
                    .push(widget::text(label))
                    .push(
                        widget::text_input("", value)
                            .on_input(on_input)
                            .width(Length::Fill),
                    )
            };
            // A per-peer text field whose message carries the peer index.
            let peer_field = |label: String,
                              value: String,
                              index: usize,
                              on_input: fn(usize, String) -> Message| {
                widget::Column::new()
                    .spacing(spacing.space_xxs)
                    .push(widget::text(label))
                    .push(
                        widget::text_input("", value)
                            .on_input(move |v| on_input(index, v))
                            .width(Length::Fill),
                    )
            };
            // Masked variants for secret keys (private key, pre-shared key).
            let secret_field = |label: String, value: String, on_input: fn(String) -> Message| {
                widget::Column::new()
                    .spacing(spacing.space_xxs)
                    .push(widget::text(label))
                    .push(
                        widget::secure_input("", value, None, true)
                            .on_input(on_input)
                            .width(Length::Fill),
                    )
            };
            let secret_peer_field =
                |label: String,
                 value: String,
                 index: usize,
                 on_input: fn(usize, String) -> Message| {
                    widget::Column::new()
                        .spacing(spacing.space_xxs)
                        .push(widget::text(label))
                        .push(
                            widget::secure_input("", value, None, true)
                                .on_input(move |v| on_input(index, v))
                                .width(Length::Fill),
                        )
                };
            let title = if editor.edit_uuid.is_some() {
                fl!("edit-vpn", kind = "WireGuard".to_string())
            } else {
                fl!("new-vpn", kind = "WireGuard".to_string())
            };
            let mut wg = widget::Column::new()
                .spacing(spacing.space_xs)
                .padding([spacing.space_xxs, spacing.space_m])
                .push(widget::text(title))
                .push(field(
                    fl!("name-placeholder"),
                    editor.name.clone(),
                    Message::EditorName,
                ))
                .push(widget::settings::item(
                    fl!("autoconnect"),
                    widget::toggler(editor.autoconnect).on_toggle(Message::EditorAutoconnect),
                ))
                .push(secret_field(
                    fl!("wg-private-key"),
                    editor.wg_private_key.clone(),
                    |v| Message::EditorWgPrivateKey(Secret(v)),
                ))
                .push(field(
                    fl!("wg-address"),
                    editor.wg_address.clone(),
                    Message::EditorWgAddress,
                ))
                .push(field(
                    fl!("dns-servers"),
                    editor.wg_dns.clone(),
                    Message::EditorWgDns,
                ))
                .push(field(
                    fl!("wg-mtu"),
                    editor.wg_mtu.clone(),
                    Message::EditorWgMtu,
                ));

            let allow_remove = editor.wg_peers.len() > 1;
            for (index, peer) in editor.wg_peers.iter().enumerate() {
                let mut header = widget::Row::new()
                    .align_y(Alignment::Center)
                    .spacing(spacing.space_xs)
                    .push(widget::text(fl!(
                        "wg-peer-n",
                        index = (index + 1).to_string()
                    )));
                if allow_remove {
                    header = header.push(
                        widget::button::text(fl!("remove"))
                            .on_press(Message::EditorWgRemovePeer(index)),
                    );
                }
                wg = wg
                    .push(header)
                    .push(peer_field(
                        fl!("wg-peer-key"),
                        peer.public_key.clone(),
                        index,
                        Message::EditorWgPeerKey,
                    ))
                    .push(peer_field(
                        fl!("wg-peer-endpoint"),
                        peer.endpoint.clone(),
                        index,
                        Message::EditorWgPeerEndpoint,
                    ))
                    .push(peer_field(
                        fl!("wg-allowed-ips"),
                        peer.allowed_ips.clone(),
                        index,
                        Message::EditorWgAllowedIps,
                    ))
                    .push(secret_peer_field(
                        fl!("wg-preshared-key"),
                        peer.preshared_key.clone(),
                        index,
                        |i, v| Message::EditorWgPeerPsk(i, Secret(v)),
                    ))
                    .push(peer_field(
                        fl!("wg-keepalive"),
                        peer.persistent_keepalive.clone(),
                        index,
                        Message::EditorWgPeerKeepalive,
                    ));
            }
            wg = wg
                .push(widget::button::text(fl!("wg-add-peer")).on_press(Message::EditorWgAddPeer));

            let can_save = !editor.name.trim().is_empty() && !editor.saving;
            wg = wg.push(
                widget::Row::with_children(vec![
                    widget::button::standard(fl!("cancel"))
                        .on_press(Message::EditorCancel)
                        .into(),
                    widget::button::suggested(fl!("save"))
                        .on_press_maybe(can_save.then_some(Message::EditorSave))
                        .into(),
                ])
                .spacing(spacing.space_xs),
            );
            return widget::scrollable(wg).into();
        }

        // Step 2: the form.
        let title = if editor.edit_uuid.is_some() {
            fl!("edit-vpn", kind = editor.type_name.clone())
        } else {
            fl!("new-vpn", kind = editor.type_name.clone())
        };
        column = column.push(widget::text(title));
        column = column.push(
            widget::text_input(fl!("name-placeholder"), editor.name.clone())
                .on_input(Message::EditorName)
                .width(Length::Fill),
        );
        column = column.push(widget::settings::item(
            fl!("autoconnect"),
            widget::toggler(editor.autoconnect).on_toggle(Message::EditorAutoconnect),
        ));

        // General settings: autoconnect priority, metered mode, firewall zone.
        column = column.push(widget::settings::item(
            fl!("priority"),
            widget::text_input("", editor.autoconnect_priority.clone())
                .on_input(Message::EditorPriority)
                .width(Length::Fixed(96.0)),
        ));
        {
            let metered = editor.metered;
            let metered_button = |value: i32, label: String| {
                let button = if metered == value {
                    widget::button::suggested(label)
                } else {
                    widget::button::standard(label)
                };
                button.on_press(Message::EditorMetered(value))
            };
            column = column.push(
                widget::Row::with_children(vec![
                    widget::text(fl!("metered")).width(Length::Fill).into(),
                    metered_button(0, fl!("metered-auto")).into(),
                    metered_button(1, fl!("metered-yes")).into(),
                    metered_button(2, fl!("metered-no")).into(),
                ])
                .align_y(Alignment::Center)
                .spacing(spacing.space_xxs),
            );
        }
        column = column.push(widget::settings::item(
            fl!("zone"),
            widget::text_input("", editor.zone.clone())
                .on_input(Message::EditorZone)
                .width(Length::Fixed(160.0)),
        ));
        column = column.push(widget::settings::item(
            fl!("split-tunnel"),
            widget::toggler(editor.never_default).on_toggle(Message::EditorNeverDefault),
        ));
        column = column.push(widget::settings::item(
            fl!("dns-search"),
            widget::text_input("", editor.dns_search.clone())
                .on_input(Message::EditorDnsSearch)
                .width(Length::Fixed(200.0)),
        ));
        column = column.push(widget::settings::item(
            fl!("dns-servers"),
            widget::text_input("", editor.dns.clone())
                .on_input(Message::EditorDns)
                .width(Length::Fixed(200.0)),
        ));

        let service_type = editor.service_type.as_deref().unwrap_or_default();
        if service_type.ends_with("openconnect") {
            // Structured OpenConnect form (mirrors plasma-nm's layout), bound to
            // the plugin's vpn.data keys. Informed by plasma-nm's openconnect
            // widget; see docs/ATTRIBUTION.md — no code copied.
            let text_field = |label: String, key: &'static str| -> Element<'_, Message> {
                widget::settings::item(
                    label,
                    widget::text_input("", editor.data_value(key))
                        .on_input(move |v| Message::EditorDataSet(key.to_string(), v))
                        .width(Length::Fixed(260.0)),
                )
                .into()
            };
            let file_field = |label: String, key: &'static str| -> Element<'_, Message> {
                widget::settings::item(
                    label,
                    widget::Row::with_children(vec![
                        widget::text_input("", editor.data_value(key))
                            .on_input(move |v| Message::EditorDataSet(key.to_string(), v))
                            .width(Length::Fixed(220.0))
                            .into(),
                        cosmic::applet::menu_button(
                            widget::icon::from_name("document-open-symbolic")
                                .size(16)
                                .symbolic(true),
                        )
                        .width(Length::Shrink)
                        .on_press(Message::EditorBrowseDataKey(key.to_string()))
                        .into(),
                    ])
                    .spacing(spacing.space_xxs)
                    .align_y(Alignment::Center),
                )
                .into()
            };
            let toggle_field = |label: String, key: &'static str| -> Element<'_, Message> {
                let on = editor.data_value(key) == "yes";
                widget::settings::item(
                    label,
                    widget::toggler(on).on_toggle(move |v| {
                        let value = if v { "yes".to_string() } else { String::new() };
                        Message::EditorDataSet(key.to_string(), value)
                    }),
                )
                .into()
            };

            // VPN Protocol dropdown (display label -> NM `protocol` value).
            const PROTOCOL_VALUES: [&str; 7] =
                ["anyconnect", "nc", "gp", "pulse", "f5", "fortinet", "array"];
            let protocol_labels = vec![
                "Cisco AnyConnect".to_string(),
                "Juniper Network Connect".to_string(),
                "PAN GlobalProtect".to_string(),
                "Pulse Connect Secure".to_string(),
                "F5 BIG-IP".to_string(),
                "Fortinet Fortigate".to_string(),
                "Array SSL VPN".to_string(),
            ];
            let protocol_cur = editor.data_value("protocol");
            let protocol_selected = PROTOCOL_VALUES
                .iter()
                .position(|v| *v == protocol_cur)
                .or(Some(0));
            let protocol = widget::settings::item(
                fl!("oc-protocol"),
                widget::dropdown(protocol_labels, protocol_selected, move |i| {
                    Message::EditorDataSet("protocol".to_string(), PROTOCOL_VALUES[i].to_string())
                })
                .width(Length::Fixed(260.0)),
            );

            // Reported OS dropdown ("" = default / unset).
            const OS_VALUES: [&str; 7] = [
                "",
                "linux",
                "linux-64",
                "win",
                "mac-intel",
                "android",
                "apple-ios",
            ];
            let os_labels = vec![
                fl!("oc-os-default"),
                "Linux".to_string(),
                "Linux 64-bit".to_string(),
                "Windows".to_string(),
                "Mac Intel".to_string(),
                "Android".to_string(),
                "Apple iOS".to_string(),
            ];
            let os_cur = editor.data_value("reported_os");
            let os_selected = OS_VALUES.iter().position(|v| *v == os_cur).or(Some(0));
            let reported_os = widget::settings::item(
                fl!("oc-reported-os"),
                widget::dropdown(os_labels, os_selected, move |i| {
                    Message::EditorDataSet("reported_os".to_string(), OS_VALUES[i].to_string())
                })
                .width(Length::Fixed(260.0)),
            );

            column = column
                .push(widget::text(fl!("oc-general")))
                .push(protocol)
                .push(text_field(fl!("detail-gateway"), "gateway"))
                .push(file_field(fl!("oc-ca-cert"), "cacert"))
                .push(text_field(fl!("oc-proxy"), "proxy"))
                .push(text_field(fl!("oc-user-agent"), "useragent"))
                .push(text_field(fl!("oc-reported-version"), "version_string"))
                .push(reported_os)
                .push(toggle_field(fl!("oc-csd-trojan"), "enable_csd_trojan"))
                .push(file_field(fl!("oc-csd-wrapper"), "csd_wrapper"))
                .push(widget::text(fl!("oc-cert-auth")))
                .push(file_field(fl!("oc-machine-cert"), "mcacert"))
                .push(file_field(fl!("oc-private-key"), "mcakey"))
                .push(file_field(fl!("oc-user-cert"), "usercert"))
                .push(file_field(fl!("oc-private-key"), "userkey"))
                .push(toggle_field(fl!("oc-fsid"), "pem_passphrase_fsid"))
                .push(toggle_field(
                    fl!("oc-prevent-invalid"),
                    "prevent_invalid_cert",
                ));
        } else if service_type.ends_with("openvpn") {
            // Structured OpenVPN form; the auth fields depend on the connection type.
            let ct = editor.data_value("connection-type");
            let ct = if ct.is_empty() { "tls".to_string() } else { ct };
            const CT_VALUES: [&str; 4] = ["tls", "password", "password-tls", "static-key"];
            let ct_labels = vec![
                fl!("ovpn-ct-tls"),
                fl!("ovpn-ct-password"),
                fl!("ovpn-ct-password-tls"),
                fl!("ovpn-ct-static-key"),
            ];
            column = column
                .push(widget::text(fl!("oc-general")))
                .push(data_text_field(
                    fl!("detail-gateway"),
                    "remote",
                    editor.data_value("remote"),
                ))
                .push(data_choice_field(
                    fl!("ovpn-conn-type"),
                    "connection-type",
                    &CT_VALUES,
                    ct_labels,
                    ct.clone(),
                ));
            if ct != "static-key" {
                column = column.push(data_file_field(
                    fl!("oc-ca-cert"),
                    "ca",
                    editor.data_value("ca"),
                ));
            }
            if ct == "tls" || ct == "password-tls" {
                column = column
                    .push(data_file_field(
                        fl!("oc-user-cert"),
                        "cert",
                        editor.data_value("cert"),
                    ))
                    .push(data_file_field(
                        fl!("oc-private-key"),
                        "key",
                        editor.data_value("key"),
                    ));
            }
            if ct == "password" || ct == "password-tls" {
                column = column.push(data_text_field(
                    fl!("ovpn-username"),
                    "username",
                    editor.data_value("username"),
                ));
            }
            if ct == "static-key" {
                const DIR_VALUES: [&str; 3] = ["", "0", "1"];
                let dir_labels = vec![fl!("oc-os-default"), "0".to_string(), "1".to_string()];
                column = column
                    .push(data_file_field(
                        fl!("ovpn-static-key"),
                        "static-key",
                        editor.data_value("static-key"),
                    ))
                    .push(data_choice_field(
                        fl!("ovpn-key-dir"),
                        "static-key-direction",
                        &DIR_VALUES,
                        dir_labels,
                        editor.data_value("static-key-direction"),
                    ))
                    .push(data_text_field(
                        fl!("ovpn-remote-ip"),
                        "remote-ip",
                        editor.data_value("remote-ip"),
                    ))
                    .push(data_text_field(
                        fl!("ovpn-local-ip"),
                        "local-ip",
                        editor.data_value("local-ip"),
                    ));
            }
            column = column
                .push(widget::text(fl!("ovpn-advanced")))
                .push(data_text_field(
                    fl!("ovpn-port"),
                    "port",
                    editor.data_value("port"),
                ))
                .push(data_toggle_field(
                    fl!("ovpn-tcp"),
                    "proto-tcp",
                    editor.data_value("proto-tcp") == "yes",
                ))
                .push(data_text_field(
                    fl!("ovpn-cipher"),
                    "cipher",
                    editor.data_value("cipher"),
                ))
                .push(data_enum_field(
                    fl!("ovpn-auth"),
                    "auth",
                    service_type,
                    editor.data_value("auth"),
                ));
        } else if service_type.ends_with("vpnc") {
            // Structured vpnc (Cisco IPsec) form.
            column = column
                .push(widget::text(fl!("oc-general")))
                .push(data_text_field(
                    fl!("detail-gateway"),
                    "IPSec gateway",
                    editor.data_value("IPSec gateway"),
                ))
                .push(data_text_field(
                    fl!("vpnc-group"),
                    "IPSec ID",
                    editor.data_value("IPSec ID"),
                ))
                .push(data_text_field(
                    fl!("ovpn-username"),
                    "Xauth username",
                    editor.data_value("Xauth username"),
                ))
                .push(data_text_field(
                    fl!("vpnc-domain"),
                    "Domain",
                    editor.data_value("Domain"),
                ))
                .push(data_enum_field(
                    fl!("vpnc-nat"),
                    "NAT Traversal Mode",
                    service_type,
                    editor.data_value("NAT Traversal Mode"),
                ))
                .push(data_enum_field(
                    fl!("vpnc-pfs"),
                    "Perfect Forward Secrecy",
                    service_type,
                    editor.data_value("Perfect Forward Secrecy"),
                ))
                .push(data_enum_field(
                    fl!("vpnc-dh"),
                    "IKE DH Group",
                    service_type,
                    editor.data_value("IKE DH Group"),
                ))
                .push(data_enum_field(
                    fl!("vpnc-vendor"),
                    "Vendor",
                    service_type,
                    editor.data_value("Vendor"),
                ))
                .push(data_text_field(
                    fl!("vpnc-app-version"),
                    "Application Version",
                    editor.data_value("Application Version"),
                ))
                .push(data_toggle_field(
                    fl!("vpnc-single-des"),
                    "Enable Single DES",
                    editor.data_value("Enable Single DES") == "yes",
                ));
        } else {
            column = column.push(widget::text(fl!("advanced-data")));
            for (index, (key, value)) in editor.data.iter().enumerate() {
                // Secret-storage flag keys (`*-flags`) get a labelled dropdown mapping to
                // NM's numeric flags; closed-enum keys get a raw-value dropdown; anything
                // else (or an off-list value from an existing profile) stays free-form.
                let value_element: Element<'_, Message> = if key.ends_with("-flags") {
                    // NMSettingSecretFlags is a bitfield: 0 = NONE (stored), 1 =
                    // AGENT_OWNED (also stored), 2 = NOT_SAVED (ask every time), 4 =
                    // NOT_REQUIRED. Map the three user choices to 0 / 2 / 4.
                    const FLAG_VALUES: [&str; 3] = ["0", "2", "4"];
                    let labels = [fl!("secret-store"), fl!("secret-ask"), fl!("secret-none")];
                    let selected = match value.trim() {
                        "2" => Some(1),
                        "4" => Some(2),
                        _ => Some(0), // 0 (NONE) and 1 (AGENT_OWNED) are both "stored"
                    };
                    widget::dropdown(labels.to_vec(), selected, move |i| {
                        Message::EditorDataValue(index, FLAG_VALUES[i].to_string())
                    })
                    .width(Length::Fill)
                    .into()
                } else {
                    match crate::backend::enums::enum_values(service_type, key) {
                        Some(options) if value.is_empty() || options.contains(&value.as_str()) => {
                            let selected = options.iter().position(|o| *o == value.as_str());
                            widget::dropdown(options, selected, move |i| {
                                Message::EditorDataValue(index, options[i].to_string())
                            })
                            .width(Length::Fill)
                            .into()
                        }
                        _ => widget::text_input(fl!("value-placeholder"), value.clone())
                            .on_input(move |v| Message::EditorDataValue(index, v))
                            .width(Length::Fill)
                            .into(),
                    }
                };
                let row = widget::Row::with_children(vec![
                    widget::text_input(fl!("key-placeholder"), key.clone())
                        .on_input(move |k| Message::EditorDataKey(index, k))
                        .width(Length::Fill)
                        .into(),
                    value_element,
                    cosmic::applet::menu_button(
                        widget::icon::from_name("document-open-symbolic")
                            .size(16)
                            .symbolic(true),
                    )
                    .width(Length::Shrink)
                    .on_press(Message::EditorBrowseData(index))
                    .into(),
                    cosmic::applet::menu_button(
                        widget::icon::from_name("list-remove-symbolic")
                            .size(16)
                            .symbolic(true),
                    )
                    .width(Length::Shrink)
                    .on_press(Message::EditorRemoveRow(index))
                    .into(),
                ])
                .spacing(spacing.space_xxs)
                .align_y(Alignment::Center);
                column = column.push(row);
            }
            column = column.push(
                cosmic::applet::menu_button(
                    widget::Row::with_children(vec![
                        widget::icon::from_name("list-add-symbolic")
                            .size(16)
                            .symbolic(true)
                            .into(),
                        widget::text(fl!("add-field")).into(),
                    ])
                    .align_y(Alignment::Center)
                    .spacing(spacing.space_s),
                )
                .on_press(Message::EditorAddRow),
            );
        }

        let cancel = widget::button::standard(fl!("cancel")).on_press(Message::EditorCancel);
        let can_save = !editor.name.trim().is_empty() && !editor.saving;
        let save = widget::button::suggested(fl!("save"))
            .on_press_maybe(can_save.then_some(Message::EditorSave));
        column = column.push(
            widget::Row::with_children(vec![cancel.into(), save.into()]).spacing(spacing.space_xs),
        );

        widget::scrollable(column).into()
    }
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.takilazy.CosmicExtVpnMenu";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = AppModel {
            core,
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
            ..Default::default()
        };

        // Load the connection list once at startup so the panel icon/popup are ready.
        (app, load_task())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// The applet's panel button. Its icon reflects the aggregate VPN state.
    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button(panel_icon(&self.connections))
            .on_press(Message::TogglePopup)
            .into()
    }

    /// The applet's popup window: an optional secret prompt above the list of VPN
    /// connections with their state.
    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        if let Some(editor) = &self.editor {
            return self
                .core
                .applet
                .popup_container(self.editor_view(editor))
                .into();
        }

        let body: Element<'_, Message> = if let Some(error) = &self.error {
            widget::text(fl!("load-error", error = error.clone())).into()
        } else if self.connections.is_empty() {
            widget::text(fl!("no-vpn-connections")).into()
        } else {
            let mut list = widget::list_column();
            for connection in &self.connections {
                list = list.add(self.connection_view(connection));
            }
            list.into()
        };

        let spacing = cosmic::theme::active().cosmic().spacing;
        let action_button = |icon: &'static str, label: String, message: Message| {
            cosmic::applet::menu_button(
                widget::Row::with_children(vec![
                    widget::icon::from_name(icon).size(16).symbolic(true).into(),
                    widget::text(label).into(),
                ])
                .align_y(Alignment::Center)
                .spacing(spacing.space_s),
            )
            .on_press(message)
        };

        let mut content = widget::Column::new().spacing(spacing.space_xxs);
        if let Some(prompt) = &self.secret_prompt {
            content = content
                .push(self.secret_prompt_view(prompt))
                .push(widget::divider::horizontal::default());
        }
        content = content
            .push(body)
            .push(widget::divider::horizontal::default())
            .push(action_button(
                "list-add-symbolic",
                fl!("add-vpn"),
                Message::OpenCreate,
            ))
            .push(action_button(
                "document-open-symbolic",
                fl!("import"),
                Message::ImportRequested,
            ));

        self.core.applet.popup_container(content).into()
    }

    /// Register subscriptions for this application.
    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![
            // Live NetworkManager state via signals (no polling).
            nm_subscription(),
            // Our NetworkManager secret agent (password/OTP prompts).
            secret_agent_subscription(),
            // Surface VPN activation failures with their reason.
            vpn_failure_subscription(),
            // Mute notifications across system suspend/resume.
            sleep_subscription(),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
        ];

        // Tick once a second while the popup is open, to advance the duration display.
        if self.popup.is_some() {
            subscriptions
                .push(cosmic::iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick));
        }

        Subscription::batch(subscriptions)
    }

    /// Handles messages emitted by the application and its widgets.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    // The live subscription keeps state fresh; just open the popup.
                    self.open_popup()
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Loaded(connections) => {
                self.update_durations(&connections);
                self.update_traffic(&connections);
                let events = if self.notifications_ready && !self.notifications_muted() {
                    status_transitions(&self.connections, &connections)
                } else {
                    self.notifications_ready = true;
                    Vec::new()
                };
                self.connections = connections;
                self.error = None;
                if !events.is_empty() {
                    return notify_task(events);
                }
            }
            Message::LoadFailed(error) => {
                self.error = Some(error);
            }
            Message::PrepareForSleep(going_to_sleep) => {
                // Suspending: mute indefinitely (monotonic clock is frozen while
                // asleep). Resuming: keep muted for a short grace so NM's teardown
                // and re-establish churn doesn't fire connect/disconnect toasts.
                self.mute_notifications_until = Some(
                    Instant::now()
                        + if going_to_sleep {
                            Duration::from_secs(3600)
                        } else {
                            Duration::from_secs(8)
                        },
                );
            }
            Message::Activate(uuid) => {
                self.pending.insert(uuid.clone());
                self.set_status(&uuid, VpnStatus::Connecting);
                return op_task(uuid, true);
            }
            Message::Deactivate(uuid) => {
                self.pending.insert(uuid.clone());
                self.set_status(&uuid, VpnStatus::Disconnecting);
                return op_task(uuid, false);
            }
            Message::OpFinished { uuid, result } => {
                self.pending.remove(&uuid);
                if let Err(error) = result {
                    self.error = Some(error);
                }
                // Reconcile optimistic state against NetworkManager's real state.
                return load_task();
            }
            Message::ToggleExpand(uuid) => {
                self.confirm_forget = None;
                self.expanded = if self.expanded.as_deref() == Some(&uuid) {
                    None
                } else {
                    Some(uuid)
                };
            }
            Message::SetAutoconnect(uuid, on) => {
                if let Some(connection) = self.connections.iter_mut().find(|c| c.uuid == uuid) {
                    connection.autoconnect = on; // optimistic
                }
                self.pending.insert(uuid.clone());
                return autoconnect_task(uuid, on);
            }
            Message::RequestForget(uuid) => {
                self.confirm_forget = Some(uuid);
            }
            Message::CancelForget => {
                self.confirm_forget = None;
            }
            Message::ConfirmForget(uuid) => {
                self.confirm_forget = None;
                self.pending.insert(uuid.clone());
                return forget_task(uuid);
            }
            Message::Duplicate(uuid) => {
                self.error = None;
                return duplicate_task(uuid);
            }
            Message::Tick => {
                // While the popup is open with an active tunnel, refresh so the
                // duration and traffic counters advance live.
                if self.popup.is_some() && self.connections.iter().any(|c| c.status.is_active()) {
                    return load_task();
                }
            }
            Message::SecretRequested(prompt) => {
                self.secret_prompt = Some(prompt);
                // Surface the prompt even if the popup is currently closed.
                return self.open_popup();
            }
            Message::SecretInput { key, value } => {
                if let Some(prompt) = &mut self.secret_prompt
                    && let Some(field) = prompt.fields.iter_mut().find(|f| f.key == key)
                {
                    field.value = value;
                }
            }
            Message::ToggleSecretVisibility(key) => {
                if let Some(prompt) = &mut self.secret_prompt
                    && let Some(field) = prompt.fields.iter_mut().find(|f| f.key == key)
                {
                    field.hidden = !field.hidden;
                }
            }
            Message::SubmitSecret => {
                if let Some(prompt) = self.secret_prompt.take() {
                    let secrets: HashMap<String, String> = prompt
                        .fields
                        .iter()
                        .map(|f| (f.key.clone(), f.value.0.clone()))
                        .collect();
                    let responder = prompt.responder.0.clone();
                    return cosmic::task::future(async move {
                        if let Some(responder) = responder.lock().await.take() {
                            let _ = responder.vpn_secrets(secrets).await;
                        }
                        Message::SecretDone
                    })
                    .map(cosmic::Action::App);
                }
            }
            Message::CancelSecret => {
                if let Some(prompt) = self.secret_prompt.take() {
                    let responder = prompt.responder.0.clone();
                    return cosmic::task::future(async move {
                        if let Some(responder) = responder.lock().await.take() {
                            let _ = responder.cancel().await;
                        }
                        Message::SecretDone
                    })
                    .map(cosmic::Action::App);
                }
            }
            Message::SecretCancelled => {
                self.secret_prompt = None;
            }
            Message::SecretDone => {}
            Message::VpnFailed { name, reason } => {
                let body = match reason_text(reason) {
                    Some(detail) => format!("{name}: {detail}"),
                    None => name.clone(),
                };
                self.error = Some(body.clone());
                if !self.notifications_muted() {
                    return notify_task(vec![(fl!("notify-failed"), body)]);
                }
            }
            Message::NotificationsSent => {}
            Message::ImportRequested => {
                self.error = None;
                return pick_import_file();
            }
            Message::ImportPicked(path) => {
                return import_task(path);
            }
            Message::ImportDismissed => {}
            Message::ImportFinished(result) => {
                // On success the live subscription refreshes the list.
                if let Err(error) = result {
                    self.error = Some(error);
                }
            }
            Message::OpenCreate => {
                self.error = None;
                self.editor = Some(Editor {
                    plugins: crate::backend::vpn_auth::list_plugins(),
                    ..Editor::default()
                });
            }
            Message::OpenEdit(uuid) => {
                self.error = None;
                let is_wireguard = self
                    .connections
                    .iter()
                    .find(|c| c.uuid == uuid)
                    .is_some_and(|c| c.kind == VpnKind::WireGuard);
                return if is_wireguard {
                    read_wireguard_task(uuid)
                } else {
                    read_task(uuid)
                };
            }
            Message::EditLoaded { uuid, result } => match result {
                Ok(edit) => {
                    let type_name = edit
                        .service_type
                        .rsplit('.')
                        .next()
                        .unwrap_or(&edit.service_type)
                        .to_string();
                    self.editor = Some(Editor {
                        edit_uuid: Some(uuid),
                        service_type: Some(edit.service_type),
                        type_name,
                        name: edit.name,
                        autoconnect: edit.autoconnect,
                        autoconnect_priority: edit.autoconnect_priority.to_string(),
                        metered: edit.metered,
                        zone: edit.zone,
                        never_default: edit.never_default,
                        dns_search: edit.dns_search,
                        dns: edit.dns,
                        data: edit.data,
                        ..Editor::default()
                    });
                }
                Err(error) => self.error = Some(error),
            },
            Message::EditorSelectType(service_type, type_name) => {
                if let Some(editor) = &mut self.editor {
                    // Default OpenConnect's protocol so the dropdown starts on
                    // Cisco AnyConnect for a new connection.
                    if service_type.ends_with("openconnect")
                        && editor.data_value("protocol").is_empty()
                    {
                        editor.set_data("protocol", "anyconnect".to_string());
                    }
                    // Default OpenVPN's connection type to certificates (TLS).
                    if service_type.ends_with("openvpn")
                        && editor.data_value("connection-type").is_empty()
                    {
                        editor.set_data("connection-type", "tls".to_string());
                    }
                    editor.service_type = Some(service_type);
                    editor.type_name = type_name;
                }
            }
            Message::EditorSelectWireGuard => {
                if let Some(editor) = &mut self.editor {
                    editor.is_wireguard = true;
                    editor.service_type = Some("wireguard".to_string());
                    editor.type_name = "WireGuard".to_string();
                    if editor.wg_peers.is_empty() {
                        editor.wg_peers.push(WgPeerForm::default());
                    }
                }
            }
            Message::WireGuardEditLoaded { uuid, result } => match result {
                Ok(edit) => {
                    let mut peers: Vec<WgPeerForm> =
                        edit.peers.into_iter().map(WgPeerForm::from_spec).collect();
                    if peers.is_empty() {
                        peers.push(WgPeerForm::default());
                    }
                    self.editor = Some(Editor {
                        edit_uuid: Some(uuid),
                        is_wireguard: true,
                        service_type: Some("wireguard".to_string()),
                        type_name: "WireGuard".to_string(),
                        name: edit.name,
                        autoconnect: edit.autoconnect,
                        wg_private_key: edit.private_key,
                        wg_address: edit.address,
                        wg_dns: edit.dns,
                        wg_mtu: edit.mtu,
                        wg_peers: peers,
                        ..Editor::default()
                    });
                }
                Err(error) => self.error = Some(error),
            },
            Message::EditorWgPrivateKey(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.wg_private_key = value.0;
                }
            }
            Message::EditorWgAddress(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.wg_address = value;
                }
            }
            Message::EditorWgDns(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.wg_dns = value;
                }
            }
            Message::EditorWgMtu(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.wg_mtu = value;
                }
            }
            Message::EditorWgAddPeer => {
                if let Some(editor) = &mut self.editor {
                    editor.wg_peers.push(WgPeerForm::default());
                }
            }
            Message::EditorWgRemovePeer(index) => {
                if let Some(editor) = &mut self.editor
                    && editor.wg_peers.len() > 1
                    && index < editor.wg_peers.len()
                {
                    editor.wg_peers.remove(index);
                }
            }
            Message::EditorWgPeerKey(index, value) => {
                if let Some(editor) = &mut self.editor
                    && let Some(peer) = editor.wg_peers.get_mut(index)
                {
                    peer.public_key = value;
                }
            }
            Message::EditorWgPeerEndpoint(index, value) => {
                if let Some(editor) = &mut self.editor
                    && let Some(peer) = editor.wg_peers.get_mut(index)
                {
                    peer.endpoint = value;
                }
            }
            Message::EditorWgAllowedIps(index, value) => {
                if let Some(editor) = &mut self.editor
                    && let Some(peer) = editor.wg_peers.get_mut(index)
                {
                    peer.allowed_ips = value;
                }
            }
            Message::EditorWgPeerPsk(index, value) => {
                if let Some(editor) = &mut self.editor
                    && let Some(peer) = editor.wg_peers.get_mut(index)
                {
                    peer.preshared_key = value.0;
                }
            }
            Message::EditorWgPeerKeepalive(index, value) => {
                if let Some(editor) = &mut self.editor
                    && let Some(peer) = editor.wg_peers.get_mut(index)
                {
                    peer.persistent_keepalive = value;
                }
            }
            Message::EditorName(name) => {
                if let Some(editor) = &mut self.editor {
                    editor.name = name;
                }
            }
            Message::EditorAutoconnect(on) => {
                if let Some(editor) = &mut self.editor {
                    editor.autoconnect = on;
                }
            }
            Message::EditorPriority(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.autoconnect_priority = value;
                }
            }
            Message::EditorMetered(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.metered = value;
                }
            }
            Message::EditorZone(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.zone = value;
                }
            }
            Message::EditorNeverDefault(on) => {
                if let Some(editor) = &mut self.editor {
                    editor.never_default = on;
                }
            }
            Message::EditorDnsSearch(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.dns_search = value;
                }
            }
            Message::EditorDns(value) => {
                if let Some(editor) = &mut self.editor {
                    editor.dns = value;
                }
            }
            Message::EditorDataKey(index, key) => {
                if let Some(editor) = &mut self.editor
                    && let Some(row) = editor.data.get_mut(index)
                {
                    row.0 = key;
                }
            }
            Message::EditorDataValue(index, value) => {
                if let Some(editor) = &mut self.editor
                    && let Some(row) = editor.data.get_mut(index)
                {
                    row.1 = value;
                }
            }
            Message::EditorBrowseData(index) => {
                return pick_data_file(index);
            }
            Message::EditorDataPicked(index, path) => {
                if let Some(editor) = &mut self.editor
                    && let Some(row) = editor.data.get_mut(index)
                {
                    row.1 = path;
                }
            }
            Message::EditorDataSet(key, value) => {
                if let Some(editor) = &mut self.editor {
                    editor.set_data(&key, value);
                }
            }
            Message::EditorBrowseDataKey(key) => {
                return pick_data_file_key(key);
            }
            Message::EditorDataKeyPicked(key, path) => {
                if let Some(editor) = &mut self.editor {
                    editor.set_data(&key, path);
                }
            }
            Message::EditorAddRow => {
                if let Some(editor) = &mut self.editor {
                    editor.data.push((String::new(), String::new()));
                }
            }
            Message::EditorRemoveRow(index) => {
                if let Some(editor) = &mut self.editor
                    && index < editor.data.len()
                {
                    editor.data.remove(index);
                }
            }
            Message::EditorSave => {
                if let Some(editor) = &mut self.editor
                    && !editor.name.trim().is_empty()
                    && !editor.saving
                {
                    editor.saving = true;
                    if editor.is_wireguard {
                        let spec = crate::backend::model::NewWireGuard {
                            name: editor.name.trim().to_string(),
                            autoconnect: editor.autoconnect,
                            private_key: editor.wg_private_key.trim().to_string(),
                            address: editor.wg_address.trim().to_string(),
                            dns: editor.wg_dns.trim().to_string(),
                            mtu: editor.wg_mtu.trim().to_string(),
                            peers: editor.wg_peers.iter().map(WgPeerForm::to_spec).collect(),
                        };
                        return match editor.edit_uuid.clone() {
                            Some(uuid) => update_wireguard_task(uuid, spec),
                            None => create_wireguard_task(spec),
                        };
                    }
                    if let Some(service_type) = editor.service_type.clone() {
                        let spec = crate::backend::model::NewVpn {
                            name: editor.name.trim().to_string(),
                            service_type,
                            autoconnect: editor.autoconnect,
                            autoconnect_priority: editor
                                .autoconnect_priority
                                .trim()
                                .parse()
                                .unwrap_or(0),
                            metered: editor.metered,
                            zone: editor.zone.trim().to_string(),
                            never_default: editor.never_default,
                            dns_search: editor.dns_search.trim().to_string(),
                            dns: editor.dns.trim().to_string(),
                            data: editor.data.clone(),
                        };
                        return match editor.edit_uuid.clone() {
                            Some(uuid) => update_task(uuid, spec),
                            None => create_task(spec),
                        };
                    }
                    editor.saving = false;
                }
            }
            Message::EditorCancel => {
                self.editor = None;
            }
            Message::EditorSaved(result) => match result {
                Ok(()) => self.editor = None, // subscription refreshes the list
                Err(error) => {
                    if let Some(editor) = &mut self.editor {
                        editor.saving = false;
                    }
                    self.error = Some(error);
                }
            },
            Message::UpdateConfig(config) => {
                self.config = config;
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
