// SPDX-License-Identifier: MPL-2.0

//! `nmrs`-backed [`VpnBackend`] implementation.
//!
//! This is the only module that touches the NetworkManager D-Bus binding crate;
//! everything above it works with our own [`model`] types.

use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;

use futures::{Stream, StreamExt};

use super::model::{self, VpnConnection, VpnKind, VpnStatus};
use super::{BackendError, VpnBackend};

/// A stream that yields `()` on every NetworkManager change event, ending when NM
/// becomes unavailable (its D-Bus name owner is lost). The nmrs event stream is
/// intentionally lossy — a tick just means "re-read state" — so the caller reloads
/// the connection list on each tick and reconnects when the stream ends.
pub async fn event_ticks() -> Result<Pin<Box<dyn Stream<Item = ()> + Send>>, BackendError> {
    let nm = nmrs::NetworkManager::new()
        .await
        .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
    let events = nm
        .network_events()
        .await
        .map_err(|e| BackendError(format!("subscribing to NetworkManager events: {e}")))?;

    // Keep `nm` owned by the stream state so its D-Bus connection outlives the loop.
    let ticks = futures::stream::unfold((nm, events), |(nm, mut events)| async move {
        events.next().await.map(|_event| ((), (nm, events)))
    });
    Ok(Box::pin(ticks))
}

/// A VPN activation failure with its NetworkManager reason code.
#[derive(Debug, Clone)]
pub struct VpnFailure {
    /// The connection's display name (best-effort; falls back to "VPN").
    pub name: String,
    /// `NM_VPN_CONNECTION_STATE_REASON_*` code.
    pub reason: u32,
}

/// Read the `Id` of an active-connection object, best-effort.
async fn read_active_connection_id(conn: &zbus::Connection, path: &str) -> Option<String> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.freedesktop.NetworkManager",
        path,
        "org.freedesktop.NetworkManager.Connection.Active",
    )
    .await
    .ok()?;
    proxy.get_property::<String>("Id").await.ok()
}

/// Stream of VPN activation failures, from NetworkManager's `VpnStateChanged` signal
/// (state = FAILED). This carries the transient reason code that the lossy
/// `network_events` stream drops.
pub async fn vpn_failure_events()
-> Result<Pin<Box<dyn Stream<Item = VpnFailure> + Send>>, BackendError> {
    use zbus::MatchRule;
    use zbus::message::Type;

    const FAILED: u32 = 6; // NM_VPN_CONNECTION_STATE_FAILED

    let conn = zbus::Connection::system()
        .await
        .map_err(|e| BackendError(format!("connecting to the system bus: {e}")))?;
    let rule = MatchRule::builder()
        .msg_type(Type::Signal)
        .interface("org.freedesktop.NetworkManager.VPN.Connection")
        .map_err(|e| BackendError(format!("building match rule: {e}")))?
        .member("VpnStateChanged")
        .map_err(|e| BackendError(format!("building match rule: {e}")))?
        .build();
    let stream = zbus::MessageStream::for_match_rule(rule, &conn, Some(16))
        .await
        .map_err(|e| BackendError(format!("subscribing to VPN state changes: {e}")))?;

    let failures = futures::stream::unfold((conn, stream), |(conn, mut stream)| async move {
        loop {
            let item = stream.next().await?;
            let Ok(message) = item else {
                continue;
            };
            let Ok((state, reason)) = message.body().deserialize::<(u32, u32)>() else {
                continue;
            };
            if state != FAILED {
                continue;
            }
            let path = message.header().path().map(|p| p.to_string());
            let name = match path {
                Some(p) => read_active_connection_id(&conn, &p)
                    .await
                    .unwrap_or_else(|| "VPN".to_string()),
                None => "VPN".to_string(),
            };
            return Some((VpnFailure { name, reason }, (conn, stream)));
        }
    });
    Ok(Box::pin(failures))
}

/// Subscribe to logind's `PrepareForSleep(b)` signal. Each item is `true` when the
/// system is about to suspend and `false` on resume. Used to mute the spurious
/// "disconnected" notifications NM emits while tearing tunnels down for sleep.
pub async fn sleep_events() -> Result<Pin<Box<dyn Stream<Item = bool> + Send>>, BackendError> {
    use zbus::MatchRule;
    use zbus::message::Type;

    let conn = zbus::Connection::system()
        .await
        .map_err(|e| BackendError(format!("connecting to the system bus: {e}")))?;
    let rule = MatchRule::builder()
        .msg_type(Type::Signal)
        .interface("org.freedesktop.login1.Manager")
        .map_err(|e| BackendError(format!("building match rule: {e}")))?
        .member("PrepareForSleep")
        .map_err(|e| BackendError(format!("building match rule: {e}")))?
        .build();
    let stream = zbus::MessageStream::for_match_rule(rule, &conn, Some(4))
        .await
        .map_err(|e| BackendError(format!("subscribing to sleep events: {e}")))?;

    let events = futures::stream::unfold((conn, stream), |(conn, mut stream)| async move {
        loop {
            let item = stream.next().await?;
            let Ok(message) = item else {
                continue;
            };
            let Ok(going_to_sleep) = message.body().deserialize::<bool>() else {
                continue;
            };
            return Some((going_to_sleep, (conn, stream)));
        }
    });
    Ok(Box::pin(events))
}

/// Register this applet as a NetworkManager secret agent, returning the handle and
/// the stream of secret requests. The identifier is our app ID (unique per session).
pub async fn register_secret_agent() -> Result<
    (
        nmrs::agent::SecretAgentHandle,
        futures::channel::mpsc::Receiver<nmrs::agent::SecretRequest>,
    ),
    BackendError,
> {
    nmrs::agent::SecretAgent::builder()
        .with_identifier("com.github.takilazy.CosmicExtVpnMenu")
        .with_capabilities(nmrs::agent::SecretAgentCapabilities::VPN_HINTS)
        .register()
        .await
        .map_err(|e| BackendError(format!("registering secret agent: {e}")))
}

/// Live NetworkManager backend. Stateless: it opens a fresh connection per call,
/// matching how the COSMIC network applet drives `nmrs`.
#[derive(Debug, Default, Clone, Copy)]
pub struct NmBackend;

impl NmBackend {
    /// Create a backend handle.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl VpnBackend for NmBackend {
    async fn list_connections(&self) -> Result<Vec<VpnConnection>, BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        let vpns = nm
            .list_vpn_connections()
            .await
            .map_err(|e| BackendError(format!("listing VPN connections: {e}")))?;

        // Enrich with data that `list_vpn_connections` doesn't carry: live IP
        // addresses (from active connections) and autoconnect (from saved profiles).
        // Both are best-effort — a failure here still yields the base list.
        let ip_map = active_ip_map(&nm).await;
        let saved = saved_info(&nm).await;

        // DNS servers are only meaningful (and only fetched) for active connections.
        let mut dns_map: HashMap<String, Vec<String>> = HashMap::new();
        for v in &vpns {
            if v.active
                && let Ok(info) = nm.get_vpn_info(&v.id).await
                && !info.dns_servers.is_empty()
            {
                dns_map.insert(v.uuid.clone(), info.dns_servers);
            }
        }

        // Traffic counters, only for active connections with a known interface.
        let mut traffic_map: HashMap<String, (u64, u64)> = HashMap::new();
        for v in &vpns {
            if v.active
                && let Some(iface) = &v.interface
                && let Some(counters) = device_traffic(iface).await
            {
                traffic_map.insert(v.uuid.clone(), counters);
            }
        }

        let mut connections: Vec<VpnConnection> = vpns
            .iter()
            .map(|v| convert(v, &ip_map, &saved, &dns_map, &traffic_map))
            .collect();
        model::sort_connections(&mut connections);
        Ok(connections)
    }

    // `SettingsPatch` is `#[non_exhaustive]`, so it must be built via Default + field
    // assignment; the struct-literal form clippy would suggest is not allowed here.
    #[allow(clippy::field_reassign_with_default)]
    async fn set_autoconnect(&self, uuid: &str, on: bool) -> Result<(), BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        let mut patch = nmrs::SettingsPatch::default();
        patch.autoconnect = Some(on);
        nm.update_saved_connection(uuid, patch)
            .await
            .map_err(|e| BackendError(format!("updating autoconnect: {e}")))
    }

    async fn forget(&self, uuid: &str) -> Result<(), BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.delete_saved_connection(uuid)
            .await
            .map_err(|e| BackendError(format!("deleting connection: {e}")))
    }

    async fn import(&self, path: &Path) -> Result<(), BackendError> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if ext == "ovpn" {
            return import_openvpn(path).await;
        }
        // `.conf` (or unknown): sniff the content to tell WireGuard from OpenVPN.
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| BackendError(format!("reading {}: {e}", path.display())))?;
        if model::looks_like_wireguard_conf(&content) {
            import_wireguard(path).await
        } else {
            import_openvpn(path).await
        }
    }

    async fn activate(&self, uuid: &str) -> Result<(), BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.connect_vpn_by_uuid(uuid)
            .await
            .map_err(|e| BackendError(format!("activating connection: {e}")))
    }

    async fn deactivate(&self, uuid: &str) -> Result<(), BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.disconnect_vpn_by_uuid(uuid)
            .await
            .map_err(|e| BackendError(format!("deactivating connection: {e}")))
    }

    async fn create_vpn(&self, spec: model::NewVpn) -> Result<(), BackendError> {
        use nmrs::builders::ConnectionBuilder;
        use zbus::zvariant::Value;

        let search = spec.search_domains();
        let dns4: Vec<u32> = spec
            .dns_v4()
            .iter()
            .map(|a| u32::from_ne_bytes(a.octets()))
            .collect();
        let dns6: Vec<Vec<u8>> = spec.dns_v6().iter().map(|a| a.octets().to_vec()).collect();
        let model::NewVpn {
            name,
            service_type,
            autoconnect,
            autoconnect_priority,
            metered,
            zone,
            never_default,
            data,
            ..
        } = spec;

        // Dynamic `vpn.data` keys live inside the `data` value (a nested a{ss}), not as
        // section keys (which must be &'static).
        let data: HashMap<String, String> = data
            .into_iter()
            .filter(|(key, _)| !key.trim().is_empty())
            .collect();
        let mut vpn_section: HashMap<&'static str, Value<'static>> = HashMap::new();
        vpn_section.insert("service-type", Value::from(service_type));
        vpn_section.insert("data", Value::from(data));

        let search_v6 = search.clone();
        let settings = ConnectionBuilder::new("vpn", name)
            .autoconnect(autoconnect)
            .autoconnect_priority(autoconnect_priority)
            .ipv4_auto()
            .ipv6_auto()
            .with_section("vpn", vpn_section)
            .update_section("connection", move |connection| {
                connection.insert("metered", Value::from(metered));
                if !zone.trim().is_empty() {
                    connection.insert("zone", Value::from(zone));
                }
            })
            .update_section("ipv4", move |ipv4| {
                ipv4.insert("never-default", Value::from(never_default));
                if !search.is_empty() {
                    ipv4.insert("dns-search", Value::from(search));
                }
                if !dns4.is_empty() {
                    ipv4.insert("dns", Value::from(dns4));
                    ipv4.insert("ignore-auto-dns", Value::from(true));
                }
            })
            .update_section("ipv6", move |ipv6| {
                ipv6.insert("never-default", Value::from(never_default));
                if !search_v6.is_empty() {
                    ipv6.insert("dns-search", Value::from(search_v6));
                }
                if !dns6.is_empty() {
                    ipv6.insert("dns", Value::from(dns6));
                    ipv6.insert("ignore-auto-dns", Value::from(true));
                }
            })
            .build();

        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.add_connection(settings)
            .await
            .map(|_| ())
            .map_err(|e| BackendError(format!("creating connection: {e}")))
    }

    async fn create_wireguard(&self, spec: model::NewWireGuard) -> Result<(), BackendError> {
        let settings = build_wireguard_settings(&spec)?;
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.add_connection(settings)
            .await
            .map(|_| ())
            .map_err(|e| BackendError(format!("creating connection: {e}")))
    }

    async fn read_wireguard(&self, uuid: &str) -> Result<model::WgEdit, BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        let raw = nm
            .get_saved_connection_raw(uuid)
            .await
            .map_err(|e| BackendError(format!("reading connection: {e}")))?;

        let connection = raw.get("connection");
        let wg = raw.get("wireguard");
        let peers = wg.map(owned_wg_peers).unwrap_or_default();
        // Address lives in whichever family is `manual`; join both for display.
        let mut addresses = raw.get("ipv4").map(owned_addresses).unwrap_or_default();
        addresses.extend(raw.get("ipv6").map(owned_addresses).unwrap_or_default());
        let mut dns = raw.get("ipv4").map(owned_dns_v4).unwrap_or_default();
        dns.extend(raw.get("ipv6").map(owned_dns_v6).unwrap_or_default());

        Ok(model::WgEdit {
            name: connection
                .and_then(|c| owned_str(c, "id"))
                .unwrap_or_default(),
            autoconnect: connection
                .and_then(|c| owned_bool(c, "autoconnect"))
                .unwrap_or(false),
            private_key: wg
                .and_then(|s| owned_str(s, "private-key"))
                .unwrap_or_default(),
            address: addresses.join(" "),
            dns: dns.join(" "),
            mtu: wg
                .and_then(|s| owned_u32(s, "mtu"))
                .filter(|&n| n > 0)
                .map(|n| n.to_string())
                .unwrap_or_default(),
            peers,
        })
    }

    async fn update_wireguard(
        &self,
        uuid: &str,
        spec: model::NewWireGuard,
    ) -> Result<(), BackendError> {
        use zbus::zvariant::OwnedValue;

        let autoconnect = spec.autoconnect;
        let name = spec.name.clone();
        // Reuse the builder's exact NM encoding (address-data, peers `aa{sv}`, dns),
        // then overlay only the technology sections — identity stays on the profile.
        let built = build_wireguard_settings(&spec)?;
        let mut overlay: HashMap<String, HashMap<String, OwnedValue>> = HashMap::new();
        for (section, keys) in built {
            if section == "connection" {
                continue;
            }
            let mut out = HashMap::new();
            for (key, value) in keys {
                let owned = OwnedValue::try_from(value)
                    .map_err(|e| BackendError(format!("encoding settings: {e}")))?;
                out.insert(key.to_string(), owned);
            }
            overlay.insert(section.to_string(), out);
        }

        let mut patch = nmrs::SettingsPatch::default();
        patch.id = Some(name);
        patch.autoconnect = Some(autoconnect);
        patch.raw_overlay = Some(overlay);

        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.update_saved_connection(uuid, patch)
            .await
            .map_err(|e| BackendError(format!("updating connection: {e}")))
    }

    async fn read_vpn(&self, uuid: &str) -> Result<model::VpnEdit, BackendError> {
        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        let raw = nm
            .get_saved_connection_raw(uuid)
            .await
            .map_err(|e| BackendError(format!("reading connection: {e}")))?;

        let connection = raw.get("connection");
        let vpn = raw.get("vpn");
        Ok(model::VpnEdit {
            name: connection
                .and_then(|c| owned_str(c, "id"))
                .unwrap_or_default(),
            autoconnect: connection
                .and_then(|c| owned_bool(c, "autoconnect"))
                .unwrap_or(false),
            autoconnect_priority: connection
                .and_then(|c| owned_i32(c, "autoconnect-priority"))
                .unwrap_or(0),
            metered: connection
                .and_then(|c| owned_i32(c, "metered"))
                .unwrap_or(0),
            zone: connection
                .and_then(|c| owned_str(c, "zone"))
                .unwrap_or_default(),
            never_default: raw
                .get("ipv4")
                .and_then(|s| owned_bool(s, "never-default"))
                .unwrap_or(false),
            dns_search: raw
                .get("ipv4")
                .map(|s| owned_str_list(s, "dns-search").join(" "))
                .unwrap_or_default(),
            dns: {
                let mut all = raw.get("ipv4").map(owned_dns_v4).unwrap_or_default();
                all.extend(raw.get("ipv6").map(owned_dns_v6).unwrap_or_default());
                all.join(" ")
            },
            service_type: vpn
                .and_then(|v| owned_str(v, "service-type"))
                .unwrap_or_default(),
            data: vpn.map(owned_data).unwrap_or_default(),
        })
    }

    // `SettingsPatch` is `#[non_exhaustive]` — build via Default + field assignment.
    #[allow(clippy::field_reassign_with_default)]
    async fn update_vpn(&self, uuid: &str, spec: model::NewVpn) -> Result<(), BackendError> {
        use zbus::zvariant::{OwnedValue, Value};

        let search = spec.search_domains();
        let dns4: Vec<u32> = spec
            .dns_v4()
            .iter()
            .map(|a| u32::from_ne_bytes(a.octets()))
            .collect();
        let dns6: Vec<Vec<u8>> = spec.dns_v6().iter().map(|a| a.octets().to_vec()).collect();
        let model::NewVpn {
            name,
            autoconnect,
            autoconnect_priority,
            metered,
            zone,
            never_default,
            data,
            ..
        } = spec;

        let data_map: HashMap<String, String> = data
            .into_iter()
            .filter(|(key, _)| !key.trim().is_empty())
            .collect();
        let encode = |value: Value<'static>| {
            OwnedValue::try_from(value).map_err(|e| BackendError(format!("encoding settings: {e}")))
        };

        let mut vpn: HashMap<String, OwnedValue> = HashMap::new();
        vpn.insert("data".to_string(), encode(Value::from(data_map))?);

        let mut connection: HashMap<String, OwnedValue> = HashMap::new();
        connection.insert("metered".to_string(), encode(Value::from(metered))?);
        connection.insert("zone".to_string(), encode(Value::from(zone))?);

        // Apply IP settings (split tunnel + search domains + DNS) to both families.
        let mut ipv4: HashMap<String, OwnedValue> = HashMap::new();
        ipv4.insert(
            "never-default".to_string(),
            encode(Value::from(never_default))?,
        );
        ipv4.insert(
            "dns-search".to_string(),
            encode(Value::from(search.clone()))?,
        );
        ipv4.insert(
            "ignore-auto-dns".to_string(),
            encode(Value::from(!dns4.is_empty()))?,
        );
        ipv4.insert("dns".to_string(), encode(Value::from(dns4))?);

        let mut ipv6: HashMap<String, OwnedValue> = HashMap::new();
        ipv6.insert(
            "never-default".to_string(),
            encode(Value::from(never_default))?,
        );
        ipv6.insert("dns-search".to_string(), encode(Value::from(search))?);
        ipv6.insert(
            "ignore-auto-dns".to_string(),
            encode(Value::from(!dns6.is_empty()))?,
        );
        ipv6.insert("dns".to_string(), encode(Value::from(dns6))?);

        let mut overlay: HashMap<String, HashMap<String, OwnedValue>> = HashMap::new();
        overlay.insert("connection".to_string(), connection);
        overlay.insert("vpn".to_string(), vpn);
        overlay.insert("ipv4".to_string(), ipv4);
        overlay.insert("ipv6".to_string(), ipv6);

        let mut patch = nmrs::SettingsPatch::default();
        patch.id = Some(name);
        patch.autoconnect = Some(autoconnect);
        patch.autoconnect_priority = Some(autoconnect_priority);
        patch.raw_overlay = Some(overlay);

        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        nm.update_saved_connection(uuid, patch)
            .await
            .map_err(|e| BackendError(format!("updating connection: {e}")))
    }

    async fn duplicate(&self, uuid: &str) -> Result<(), BackendError> {
        use zbus::zvariant::Value;

        let nm = nmrs::NetworkManager::new()
            .await
            .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
        let raw = nm
            .get_saved_connection_raw(uuid)
            .await
            .map_err(|e| BackendError(format!("reading connection: {e}")))?;

        let old_id = raw
            .get("connection")
            .and_then(|c| owned_str(c, "id"))
            .unwrap_or_default();
        let new_id = format!("{old_id} (copy)");

        // Rebuild the settings, dropping identity fields so NM assigns a fresh UUID.
        let mut settings: HashMap<&str, HashMap<&str, Value<'_>>> = HashMap::new();
        for (section, keys) in &raw {
            let mut out = HashMap::new();
            for (key, value) in keys {
                if section == "connection" && matches!(key.as_str(), "uuid" | "timestamp" | "id") {
                    continue;
                }
                let cloned = value
                    .try_clone()
                    .map_err(|e| BackendError(format!("cloning settings: {e}")))?;
                out.insert(key.as_str(), Value::from(cloned));
            }
            settings.insert(section.as_str(), out);
        }
        settings
            .entry("connection")
            .or_default()
            .insert("id", Value::from(new_id));

        nm.add_connection(settings)
            .await
            .map(|_| ())
            .map_err(|e| BackendError(format!("duplicating connection: {e}")))
    }
}

/// Extract an `i32` field from a raw settings section.
fn owned_i32(section: &HashMap<String, zbus::zvariant::OwnedValue>, key: &str) -> Option<i32> {
    section.get(key)?.try_clone().ok()?.try_into().ok()
}

/// Decode an `ipv4.dns` (`au`, native-endian) field into address strings.
fn owned_dns_v4(section: &HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<String> {
    section
        .get("dns")
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| Vec::<u32>::try_from(v).ok())
        .map(|list| {
            list.iter()
                .map(|n| std::net::Ipv4Addr::from(n.to_ne_bytes()).to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Decode an `ipv6.dns` (`aay`) field into address strings.
fn owned_dns_v6(section: &HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<String> {
    section
        .get("dns")
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| Vec::<Vec<u8>>::try_from(v).ok())
        .map(|list| {
            list.iter()
                .filter_map(|bytes| <[u8; 16]>::try_from(bytes.as_slice()).ok())
                .map(|octets| std::net::Ipv6Addr::from(octets).to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Build NM WireGuard settings from an editor spec, reusing nmrs's exact encoding
/// (address-data, peers `aa{sv}`, dns `au`/`aay`). Shared by create and update.
fn build_wireguard_settings(
    spec: &model::NewWireGuard,
) -> Result<
    HashMap<&'static str, HashMap<&'static str, zbus::zvariant::Value<'static>>>,
    BackendError,
> {
    use nmrs::WireGuardPeer;
    use nmrs::builders::WireGuardBuilder;

    let mut builder = WireGuardBuilder::new(spec.name.clone())
        .private_key(spec.private_key.clone())
        .address(spec.address.clone())
        .autoconnect(spec.autoconnect);
    for p in &spec.peers {
        let mut peer =
            WireGuardPeer::new(p.public_key.clone(), p.endpoint.clone(), p.allowed_ips());
        if let Some(psk) = p.psk() {
            peer = peer.with_preshared_key(psk);
        }
        if let Some(ka) = p.keepalive() {
            peer = peer.with_persistent_keepalive(ka);
        }
        builder = builder.add_peer(peer);
    }
    let dns = spec.dns_servers();
    if !dns.is_empty() {
        builder = builder.dns(dns);
    }
    if let Some(mtu) = spec.mtu_value() {
        builder = builder.mtu(mtu);
    }
    builder
        .build()
        .map_err(|e| BackendError(format!("building WireGuard connection: {e}")))
}

/// Decode a `wireguard.peers` (`aa{sv}`) field into editor peer rows.
fn owned_wg_peers(section: &HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<model::NewWgPeer> {
    use zbus::zvariant::Value;
    let Some(raw) = section.get("peers").and_then(|v| v.try_clone().ok()) else {
        return Vec::new();
    };
    let Ok(array) = Vec::<HashMap<String, Value<'_>>>::try_from(raw) else {
        return Vec::new();
    };
    array
        .into_iter()
        .map(|dict| {
            let get_str = |key: &str| -> String {
                dict.get(key)
                    .and_then(|v| String::try_from(v.try_clone().ok()?).ok())
                    .unwrap_or_default()
            };
            let allowed = dict
                .get("allowed-ips")
                .and_then(|v| Vec::<String>::try_from(v.try_clone().ok()?).ok())
                .unwrap_or_default()
                .join(", ");
            let keepalive = dict
                .get("persistent-keepalive")
                .and_then(|v| u32::try_from(v.try_clone().ok()?).ok())
                .filter(|&n| n > 0)
                .map(|n| n.to_string())
                .unwrap_or_default();
            model::NewWgPeer {
                public_key: get_str("public-key"),
                endpoint: get_str("endpoint"),
                allowed_ips: allowed,
                preshared_key: get_str("preshared-key"),
                persistent_keepalive: keepalive,
            }
        })
        .collect()
}

/// Decode an `ipv4`/`ipv6` `address-data` (`aa{sv}`) into `addr/prefix` strings.
fn owned_addresses(section: &HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<String> {
    use zbus::zvariant::Value;
    let Some(raw) = section.get("address-data").and_then(|v| v.try_clone().ok()) else {
        return Vec::new();
    };
    let Ok(array) = Vec::<HashMap<String, Value<'_>>>::try_from(raw) else {
        return Vec::new();
    };
    array
        .into_iter()
        .filter_map(|dict| {
            let address = dict
                .get("address")
                .and_then(|v| String::try_from(v.try_clone().ok()?).ok())?;
            let prefix = dict
                .get("prefix")
                .and_then(|v| u32::try_from(v.try_clone().ok()?).ok())
                .unwrap_or(32);
            Some(format!("{address}/{prefix}"))
        })
        .collect()
}

/// Extract a `u32` field from a raw settings section.
fn owned_u32(section: &HashMap<String, zbus::zvariant::OwnedValue>, key: &str) -> Option<u32> {
    section
        .get(key)
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| u32::try_from(v).ok())
}

/// Extract a string-array (`as`) field from a raw settings section.
fn owned_str_list(section: &HashMap<String, zbus::zvariant::OwnedValue>, key: &str) -> Vec<String> {
    section
        .get(key)
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| Vec::<String>::try_from(v).ok())
        .unwrap_or_default()
}

/// Extract a string field from a raw settings section.
fn owned_str(section: &HashMap<String, zbus::zvariant::OwnedValue>, key: &str) -> Option<String> {
    section.get(key)?.try_clone().ok()?.try_into().ok()
}

/// Extract a boolean field from a raw settings section.
fn owned_bool(section: &HashMap<String, zbus::zvariant::OwnedValue>, key: &str) -> Option<bool> {
    section.get(key)?.try_clone().ok()?.try_into().ok()
}

/// Extract the `data` `a{ss}` from a raw `vpn` section as key/value pairs.
fn owned_data(vpn: &HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<(String, String)> {
    let Some(value) = vpn.get("data").and_then(|v| v.try_clone().ok()) else {
        return Vec::new();
    };
    let map: HashMap<String, String> = value.try_into().unwrap_or_default();
    let mut data: Vec<(String, String)> = map.into_iter().collect();
    data.sort_by(|a, b| a.0.cmp(&b.0));
    data
}

/// Import an OpenVPN `.ovpn`/`.conf` via nmrs's native importer.
async fn import_openvpn(path: &Path) -> Result<(), BackendError> {
    let nm = nmrs::NetworkManager::new()
        .await
        .map_err(|e| BackendError(format!("connecting to NetworkManager: {e}")))?;
    nm.import_ovpn(path, None, None)
        .await
        .map_err(|e| BackendError(format!("importing OpenVPN config: {e}")))
}

/// Import a WireGuard `.conf` by delegating to NetworkManager's own importer (`nmcli
/// connection import type wireguard file …`).
async fn import_wireguard(path: &Path) -> Result<(), BackendError> {
    let output = tokio::process::Command::new("nmcli")
        .args(["connection", "import", "type", "wireguard", "file"])
        .arg(path)
        .output()
        .await
        .map_err(|e| BackendError(format!("running nmcli: {e}")))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(BackendError(format!(
            "nmcli import failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

type IpMap = HashMap<String, (Option<String>, Option<String>)>;

/// Build a map of UUID → (IPv4, IPv6) from the active connections. Best-effort:
/// returns empty on error. Covers plugin VPNs and native WireGuard (an "other"
/// device-typed active connection).
async fn active_ip_map(nm: &nmrs::NetworkManager) -> IpMap {
    let Ok(active) = nm.list_active_connections().await else {
        return IpMap::new();
    };
    active
        .iter()
        .filter_map(|connection| {
            use nmrs::ActiveConnection as A;
            match connection {
                A::Vpn(c) => Some((
                    c.uuid.clone(),
                    (c.ip4_address.clone(), c.ip6_address.clone()),
                )),
                A::Other(c) => Some((
                    c.uuid.clone(),
                    (c.ip4_address.clone(), c.ip6_address.clone()),
                )),
                // Wired/Wifi aren't VPNs; anything else is unmodeled.
                _ => None,
            }
        })
        .collect()
}

/// Read (RxBytes, TxBytes) for a device interface via NM's Device.Statistics,
/// enabling stats collection first. Best-effort.
async fn device_traffic(interface: &str) -> Option<(u64, u64)> {
    let conn = zbus::Connection::system().await.ok()?;
    let manager = zbus::Proxy::new(
        &conn,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    )
    .await
    .ok()?;
    let device: zbus::zvariant::OwnedObjectPath = manager
        .call("GetDeviceByIpIface", &(interface,))
        .await
        .ok()?;
    let stats = zbus::Proxy::new(
        &conn,
        "org.freedesktop.NetworkManager",
        device.as_str(),
        "org.freedesktop.NetworkManager.Device.Statistics",
    )
    .await
    .ok()?;
    // NM only accumulates counters when the refresh rate is non-zero.
    let _ = stats.set_property("RefreshRateMs", 2000u32).await;
    let rx: u64 = stats.get_property("RxBytes").await.ok()?;
    let tx: u64 = stats.get_property("TxBytes").await.ok()?;
    Some((rx, tx))
}

/// Build a map of UUID → (autoconnect, last-used timestamp) from the saved profiles.
/// Best-effort.
async fn saved_info(nm: &nmrs::NetworkManager) -> HashMap<String, (bool, u64)> {
    match nm.list_saved_connections().await {
        Ok(saved) => saved
            .iter()
            .map(|profile| {
                (
                    profile.uuid.clone(),
                    (profile.autoconnect, profile.timestamp_unix),
                )
            })
            .collect(),
        Err(_) => HashMap::new(),
    }
}

/// Derive a friendly protocol label and gateway/remote address from a VPN's type.
fn label_and_gateway(vpn_type: &nmrs::VpnType) -> (String, Option<String>) {
    use nmrs::VpnType as T;
    match vpn_type {
        T::WireGuard { endpoint, .. } => ("WireGuard".to_string(), endpoint.clone()),
        T::OpenVpn { remote, .. } => ("OpenVPN".to_string(), remote.clone()),
        T::OpenConnect { gateway, .. } => ("OpenConnect".to_string(), gateway.clone()),
        T::StrongSwan { address, .. } => ("strongSwan".to_string(), address.clone()),
        T::Pptp { gateway, .. } => ("PPTP".to_string(), gateway.clone()),
        T::L2tp { gateway, .. } => ("L2TP".to_string(), gateway.clone()),
        T::Generic {
            service_type, data, ..
        } => {
            let gateway = data
                .get("gateway")
                .or_else(|| data.get("remote"))
                .or_else(|| data.get("host"))
                .cloned();
            (label_from_service(service_type), gateway)
        }
        // Future nmrs variant we don't model yet.
        _ => ("VPN".to_string(), None),
    }
}

/// Turn an NM VPN service-type into a short label, e.g.
/// `org.freedesktop.NetworkManager.vpnc` → `vpnc`.
fn label_from_service(service_type: &str) -> String {
    service_type
        .rsplit('.')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(service_type)
        .to_string()
}

/// Convert an `nmrs` VPN record into our UI-facing type, enriched with IP and
/// autoconnect data from the accompanying maps.
fn convert(
    v: &nmrs::VpnConnection,
    ip_map: &IpMap,
    saved: &HashMap<String, (bool, u64)>,
    dns_map: &HashMap<String, Vec<String>>,
    traffic_map: &HashMap<String, (u64, u64)>,
) -> VpnConnection {
    let kind = match v.kind {
        nmrs::VpnKind::WireGuard => VpnKind::WireGuard,
        // `nmrs::VpnKind` is `#[non_exhaustive]`; treat anything else as a plugin VPN.
        _ => VpnKind::Plugin,
    };
    let service_type = if v.service_type.is_empty() {
        None
    } else {
        Some(v.service_type.clone())
    };
    let (type_label, gateway) = label_and_gateway(&v.vpn_type);
    let (ip4, ip6) = ip_map.get(&v.uuid).cloned().unwrap_or((None, None));
    let (autoconnect, last_used) = saved.get(&v.uuid).copied().unwrap_or((false, 0));

    VpnConnection {
        uuid: v.uuid.clone(),
        id: v.id.clone(),
        kind,
        status: status_from(v.active, v.state.clone()),
        service_type,
        last_used,
        type_label,
        gateway,
        ip4,
        ip6,
        dns_servers: dns_map.get(&v.uuid).cloned().unwrap_or_default(),
        rx_bytes: traffic_map.get(&v.uuid).map_or(0, |(rx, _)| *rx),
        tx_bytes: traffic_map.get(&v.uuid).map_or(0, |(_, tx)| *tx),
        autoconnect,
    }
}

/// Map an `nmrs` device state (+ active flag) onto our coarse [`VpnStatus`].
///
/// `nmrs::list_vpn_connections` reports inactive saved profiles as
/// `active = false` with a placeholder `DeviceState::Other(0)`, so the active flag
/// is the primary signal; the device state only refines *how* an active VPN is up.
fn status_from(active: bool, state: nmrs::DeviceState) -> VpnStatus {
    use nmrs::DeviceState as D;

    if !active {
        return VpnStatus::Disconnected;
    }
    match state {
        D::Activated => VpnStatus::Connected,
        D::Prepare | D::Config | D::NeedAuth | D::IpConfig | D::IpCheck | D::Secondaries => {
            VpnStatus::Connecting
        }
        D::Deactivating => VpnStatus::Disconnecting,
        D::Failed | D::Disconnected | D::Unavailable | D::Unmanaged => VpnStatus::Disconnected,
        // `Other(_)` and any future variant: active but unmapped.
        _ => VpnStatus::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nmrs::DeviceState;

    #[test]
    fn inactive_is_always_disconnected() {
        // Even if the reported device state looks "up", active=false wins.
        assert_eq!(
            status_from(false, DeviceState::Activated),
            VpnStatus::Disconnected
        );
        assert_eq!(
            status_from(false, DeviceState::Other(0)),
            VpnStatus::Disconnected
        );
    }

    #[test]
    fn active_states_map_to_expected_status() {
        assert_eq!(
            status_from(true, DeviceState::Activated),
            VpnStatus::Connected
        );
        assert_eq!(
            status_from(true, DeviceState::NeedAuth),
            VpnStatus::Connecting
        );
        assert_eq!(
            status_from(true, DeviceState::Config),
            VpnStatus::Connecting
        );
        assert_eq!(
            status_from(true, DeviceState::Deactivating),
            VpnStatus::Disconnecting
        );
        assert_eq!(
            status_from(true, DeviceState::Failed),
            VpnStatus::Disconnected
        );
        assert_eq!(
            status_from(true, DeviceState::Other(99)),
            VpnStatus::Unknown
        );
    }
}
