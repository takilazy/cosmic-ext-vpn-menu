// SPDX-License-Identifier: MPL-2.0

//! UI-facing VPN types, decoupled from the D-Bus binding crate so they can be
//! constructed freely in tests and fakes (the `nmrs` models are `#[non_exhaustive]`
//! and cannot be built outside that crate).

/// Whether a connection is a NetworkManager VPN plugin or a native WireGuard tunnel.
///
/// The distinction matters because WireGuard is modeled by NetworkManager as its
/// own device type (since NM 1.16), not as a `vpn`-type connection served by a
/// plugin — see `docs/RESEARCH.md` §3.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnKind {
    /// A VPN served by an NM plugin (OpenVPN, OpenConnect, WireGuard-via-plugin, …).
    Plugin,
    /// A native (kernel) WireGuard tunnel.
    WireGuard,
}

/// Coarse connection status, collapsed from NetworkManager's finer state machine
/// for display. Failure reasons are deferred to M3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnStatus {
    /// Not active.
    Disconnected,
    /// Activating (any of prepare / need-auth / connect / ip-config).
    Connecting,
    /// Fully activated.
    Connected,
    /// Deactivating.
    Disconnecting,
    /// Active but in a state we don't map explicitly.
    Unknown,
}

impl VpnStatus {
    /// True when the connection is up or transitioning (not plain disconnected).
    #[must_use]
    pub fn is_active(self) -> bool {
        matches!(
            self,
            Self::Connecting | Self::Connected | Self::Disconnecting | Self::Unknown
        )
    }
}

/// A single VPN or WireGuard connection profile with its current status and detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpnConnection {
    /// NetworkManager connection UUID (stable identity).
    pub uuid: String,
    /// Human-visible name (`connection.id`).
    pub id: String,
    /// Plugin VPN vs native WireGuard.
    pub kind: VpnKind,
    /// Current coarse status.
    pub status: VpnStatus,
    /// `vpn.service-type` for plugin VPNs (e.g. `org.freedesktop.NetworkManager.openvpn`);
    /// `None` for WireGuard.
    pub service_type: Option<String>,
    /// Last successful activation, as Unix seconds (`connection.timestamp`); `0` if
    /// never activated.
    pub last_used: u64,
    /// Friendly protocol label, e.g. `OpenVPN`, `WireGuard`, `OpenConnect`.
    pub type_label: String,
    /// Gateway / remote endpoint, when known.
    pub gateway: Option<String>,
    /// Assigned IPv4 address (CIDR), when connected.
    pub ip4: Option<String>,
    /// Assigned IPv6 address (CIDR), when connected.
    pub ip6: Option<String>,
    /// DNS servers in use, when connected.
    pub dns_servers: Vec<String>,
    /// Bytes received on the VPN device, when connected (`0` otherwise).
    pub rx_bytes: u64,
    /// Bytes sent on the VPN device, when connected (`0` otherwise).
    pub tx_bytes: u64,
    /// Whether NetworkManager may auto-activate this profile.
    pub autoconnect: bool,
}

/// A new/edited plugin VPN (`connection.type = "vpn"`) with its general settings.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewVpn {
    /// Connection name (`connection.id`).
    pub name: String,
    /// `vpn.service-type` (the plugin).
    pub service_type: String,
    /// Whether NetworkManager may auto-activate it.
    pub autoconnect: bool,
    /// `connection.autoconnect-priority`.
    pub autoconnect_priority: i32,
    /// `connection.metered`: 0 = automatic, 1 = metered, 2 = not metered.
    pub metered: i32,
    /// `connection.zone` (firewall zone); empty = unset.
    pub zone: String,
    /// `ipv4.never-default` + `ipv6.never-default`: use this connection only for
    /// resources on its own network (split tunnel).
    pub never_default: bool,
    /// DNS search domains (space/comma separated), applied to ipv4 + ipv6.
    pub dns_search: String,
    /// DNS servers (space/comma separated IPv4/IPv6), applied to ipv4 + ipv6.
    pub dns: String,
    /// `vpn.data` key/value pairs (empty keys are dropped).
    pub data: Vec<(String, String)>,
}

impl NewVpn {
    /// Parse the search-domains field into a list.
    #[must_use]
    pub fn search_domains(&self) -> Vec<String> {
        split_fields(&self.dns_search)
    }

    /// Parse the DNS-servers field into IPv4 addresses.
    #[must_use]
    pub fn dns_v4(&self) -> Vec<std::net::Ipv4Addr> {
        split_fields(&self.dns)
            .iter()
            .filter_map(|s| s.parse::<std::net::IpAddr>().ok())
            .filter_map(|ip| match ip {
                std::net::IpAddr::V4(v) => Some(v),
                std::net::IpAddr::V6(_) => None,
            })
            .collect()
    }

    /// Parse the DNS-servers field into IPv6 addresses.
    #[must_use]
    pub fn dns_v6(&self) -> Vec<std::net::Ipv6Addr> {
        split_fields(&self.dns)
            .iter()
            .filter_map(|s| s.parse::<std::net::IpAddr>().ok())
            .filter_map(|ip| match ip {
                std::net::IpAddr::V6(v) => Some(v),
                std::net::IpAddr::V4(_) => None,
            })
            .collect()
    }
}

/// Split a space/comma/newline separated field into trimmed non-empty items.
fn split_fields(value: &str) -> Vec<String> {
    value
        .split([',', ' ', '\n', '\t'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

/// One peer of a WireGuard tunnel, as entered in the editor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewWgPeer {
    /// Peer public key (base64).
    pub public_key: String,
    /// Peer endpoint (`host:port`).
    pub endpoint: String,
    /// Peer allowed-IPs, comma/space/newline separated.
    pub allowed_ips: String,
    /// Pre-shared key (base64); empty = none.
    pub preshared_key: String,
    /// Persistent keepalive in seconds; empty/`0` = disabled.
    pub persistent_keepalive: String,
}

impl NewWgPeer {
    /// Parse the allowed-IPs field into a list, defaulting to `0.0.0.0/0` when empty.
    #[must_use]
    pub fn allowed_ips(&self) -> Vec<String> {
        let list = split_fields(&self.allowed_ips);
        if list.is_empty() {
            vec!["0.0.0.0/0".to_string()]
        } else {
            list
        }
    }

    /// Pre-shared key, or `None` when blank.
    #[must_use]
    pub fn psk(&self) -> Option<String> {
        let key = self.preshared_key.trim();
        (!key.is_empty()).then(|| key.to_string())
    }

    /// Persistent keepalive in seconds, or `None` when unset/zero/invalid.
    #[must_use]
    pub fn keepalive(&self) -> Option<u32> {
        self.persistent_keepalive
            .trim()
            .parse::<u32>()
            .ok()
            .filter(|&n| n > 0)
    }
}

/// A new/edited WireGuard tunnel (interface + one or more peers).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewWireGuard {
    /// Connection name.
    pub name: String,
    /// Autoconnect.
    pub autoconnect: bool,
    /// Interface private key (base64).
    pub private_key: String,
    /// Interface address with CIDR (e.g. `10.0.0.2/24`).
    pub address: String,
    /// DNS servers (space/comma separated) applied when connected.
    pub dns: String,
    /// Interface MTU; empty = unset.
    pub mtu: String,
    /// Peers (at least one required to build).
    pub peers: Vec<NewWgPeer>,
}

impl NewWireGuard {
    /// Parse the DNS-servers field into a list.
    #[must_use]
    pub fn dns_servers(&self) -> Vec<String> {
        split_fields(&self.dns)
    }

    /// Interface MTU, or `None` when unset/invalid.
    #[must_use]
    pub fn mtu_value(&self) -> Option<u32> {
        self.mtu.trim().parse::<u32>().ok().filter(|&n| n > 0)
    }
}

/// Current editable fields of a saved WireGuard tunnel, for prefilling the editor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WgEdit {
    /// Connection name.
    pub name: String,
    /// Autoconnect.
    pub autoconnect: bool,
    /// Interface private key (base64).
    pub private_key: String,
    /// Interface address with CIDR.
    pub address: String,
    /// DNS servers, space-separated.
    pub dns: String,
    /// Interface MTU, or empty.
    pub mtu: String,
    /// Peers.
    pub peers: Vec<NewWgPeer>,
}

/// Current editable fields of a saved plugin VPN, for prefilling the editor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VpnEdit {
    /// Connection name.
    pub name: String,
    /// Autoconnect.
    pub autoconnect: bool,
    /// `connection.autoconnect-priority`.
    pub autoconnect_priority: i32,
    /// `connection.metered` (0 automatic / 1 metered / 2 not metered).
    pub metered: i32,
    /// `connection.zone` (firewall zone).
    pub zone: String,
    /// `ipv4.never-default` (split tunnel).
    pub never_default: bool,
    /// DNS search domains, space-separated.
    pub dns_search: String,
    /// DNS servers, space-separated.
    pub dns: String,
    /// `vpn.service-type`.
    pub service_type: String,
    /// `vpn.data` key/value pairs.
    pub data: Vec<(String, String)>,
}

/// What a click on a connection row should do, given its current status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleAction {
    /// Bring the connection up.
    Activate,
    /// Take the connection down.
    Deactivate,
}

/// Decide the action for toggling a row: anything active (or transitioning) is taken
/// down; anything disconnected is brought up.
#[must_use]
pub fn toggle_action(status: VpnStatus) -> ToggleAction {
    if status.is_active() {
        ToggleAction::Deactivate
    } else {
        ToggleAction::Activate
    }
}

/// Aggregate status across all connections, used to pick the panel icon.
///
/// Connected wins over connecting, which wins over disconnected; an empty list is
/// Disconnected.
#[must_use]
pub fn overall_status(connections: &[VpnConnection]) -> VpnStatus {
    if connections.iter().any(|c| c.status == VpnStatus::Connected) {
        VpnStatus::Connected
    } else if connections.iter().any(|c| {
        matches!(
            c.status,
            VpnStatus::Connecting | VpnStatus::Disconnecting | VpnStatus::Unknown
        )
    }) {
        VpnStatus::Connecting
    } else {
        VpnStatus::Disconnected
    }
}

/// Format a byte count as a compact human-readable string (`B`/`KB`/`MB`/`GB`).
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    #[allow(clippy::cast_precision_loss)]
    let value = bytes as f64;
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", value / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", value / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", value / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Heuristic used to route imports: a WireGuard `.conf` has an `[Interface]` section;
/// OpenVPN configs never do.
#[must_use]
pub fn looks_like_wireguard_conf(content: &str) -> bool {
    content
        .lines()
        .any(|line| line.trim().eq_ignore_ascii_case("[interface]"))
}

/// Format an elapsed duration in seconds as a compact `1h 02m 03s` / `2m 03s` / `9s`.
#[must_use]
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{hours}h {minutes:02}m {secs:02}s")
    } else if minutes > 0 {
        format!("{minutes}m {secs:02}s")
    } else {
        format!("{secs}s")
    }
}

/// Orders connections for display: active/transitioning first, then most-recently
/// used, then case-insensitive by name. Stable so equal keys keep their order.
pub fn sort_connections(connections: &mut [VpnConnection]) {
    connections.sort_by(|a, b| {
        b.status
            .is_active()
            .cmp(&a.status.is_active())
            .then_with(|| b.last_used.cmp(&a.last_used))
            .then_with(|| a.id.to_lowercase().cmp(&b.id.to_lowercase()))
    });
}

/// Format a "last used" time relative to `now` (both Unix seconds). Returns `None`
/// when never used (`0`).
#[must_use]
pub fn format_last_used(last_used: u64, now: u64) -> Option<String> {
    if last_used == 0 {
        return None;
    }
    let secs = now.saturating_sub(last_used);
    Some(if secs < 60 {
        "just now".to_string()
    } else if secs < 3600 {
        let m = secs / 60;
        format!("{m} min ago")
    } else if secs < 86_400 {
        let h = secs / 3600;
        format!("{h} h ago")
    } else {
        let d = secs / 86_400;
        format!("{d} d ago")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conn(id: &str, status: VpnStatus) -> VpnConnection {
        VpnConnection {
            uuid: format!("uuid-{id}"),
            id: id.to_string(),
            kind: VpnKind::Plugin,
            status,
            service_type: None,
            last_used: 0,
            type_label: "OpenVPN".to_string(),
            gateway: None,
            ip4: None,
            ip6: None,
            dns_servers: Vec::new(),
            rx_bytes: 0,
            tx_bytes: 0,
            autoconnect: false,
        }
    }

    #[test]
    fn format_bytes_scales_units() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(2048), "2.0 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.0 MB");
    }

    #[test]
    fn format_last_used_is_relative_or_none() {
        assert_eq!(format_last_used(0, 1_000), None);
        assert_eq!(format_last_used(1_000, 1_030).as_deref(), Some("just now"));
        assert_eq!(format_last_used(1_000, 1_180).as_deref(), Some("3 min ago"));
        assert!(format_last_used(0, 7_200).is_none());
        assert_eq!(format_last_used(1, 7_201).as_deref(), Some("2 h ago"));
    }

    #[test]
    fn format_duration_scales_units() {
        assert_eq!(format_duration(9), "9s");
        assert_eq!(format_duration(125), "2m 05s");
        assert_eq!(format_duration(3725), "1h 02m 05s");
    }

    #[test]
    fn wireguard_allowed_ips_parses_and_defaults() {
        let mut peer = NewWgPeer::default();
        assert_eq!(peer.allowed_ips(), vec!["0.0.0.0/0".to_string()]);
        peer.allowed_ips = "10.0.0.0/24, ::/0\n192.168.1.0/24".to_string();
        assert_eq!(
            peer.allowed_ips(),
            vec![
                "10.0.0.0/24".to_string(),
                "::/0".to_string(),
                "192.168.1.0/24".to_string(),
            ]
        );
    }

    #[test]
    fn detects_wireguard_conf_by_interface_section() {
        assert!(looks_like_wireguard_conf(
            "[Interface]\nPrivateKey = x\n[Peer]\nPublicKey = y\n"
        ));
        assert!(looks_like_wireguard_conf("  [interface]  \n")); // whitespace / case
        assert!(!looks_like_wireguard_conf(
            "client\nremote vpn.example.com 1194\ndev tun\n"
        ));
    }

    #[test]
    fn is_active_covers_all_non_disconnected_states() {
        assert!(!VpnStatus::Disconnected.is_active());
        assert!(VpnStatus::Connecting.is_active());
        assert!(VpnStatus::Connected.is_active());
        assert!(VpnStatus::Disconnecting.is_active());
        assert!(VpnStatus::Unknown.is_active());
    }

    #[test]
    fn sort_puts_active_first_then_alphabetical() {
        let mut list = vec![
            conn("Zeta", VpnStatus::Disconnected),
            conn("beta", VpnStatus::Disconnected),
            conn("Alpha", VpnStatus::Connected),
            conn("delta", VpnStatus::Connecting),
        ];

        sort_connections(&mut list);

        let order: Vec<&str> = list.iter().map(|c| c.id.as_str()).collect();
        // Active ones ("Alpha" connected, "delta" connecting) come first, each group
        // sorted case-insensitively; then the disconnected ones ("beta", "Zeta").
        assert_eq!(order, vec!["Alpha", "delta", "beta", "Zeta"]);
    }

    #[test]
    fn overall_status_prioritizes_connected_then_connecting() {
        assert_eq!(overall_status(&[]), VpnStatus::Disconnected);
        assert_eq!(
            overall_status(&[conn("a", VpnStatus::Disconnected)]),
            VpnStatus::Disconnected
        );
        assert_eq!(
            overall_status(&[
                conn("a", VpnStatus::Disconnected),
                conn("b", VpnStatus::Connecting),
            ]),
            VpnStatus::Connecting
        );
        assert_eq!(
            overall_status(&[
                conn("a", VpnStatus::Connecting),
                conn("b", VpnStatus::Connected),
            ]),
            VpnStatus::Connected
        );
    }

    #[test]
    fn toggle_action_deactivates_active_activates_inactive() {
        assert_eq!(
            toggle_action(VpnStatus::Connected),
            ToggleAction::Deactivate
        );
        assert_eq!(
            toggle_action(VpnStatus::Connecting),
            ToggleAction::Deactivate
        );
        assert_eq!(
            toggle_action(VpnStatus::Disconnecting),
            ToggleAction::Deactivate
        );
        assert_eq!(toggle_action(VpnStatus::Unknown), ToggleAction::Deactivate);
        assert_eq!(
            toggle_action(VpnStatus::Disconnected),
            ToggleAction::Activate
        );
    }

    #[test]
    fn sort_is_case_insensitive() {
        let mut list = vec![
            conn("bravo", VpnStatus::Disconnected),
            conn("Alpha", VpnStatus::Disconnected),
        ];
        sort_connections(&mut list);
        let order: Vec<&str> = list.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(order, vec!["Alpha", "bravo"]);
    }
}
