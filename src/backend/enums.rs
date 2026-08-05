// SPDX-License-Identifier: MPL-2.0

//! Closed-enum value sets for well-known per-protocol `vpn.data` keys.
//!
//! When a connection's advanced `vpn.data` key is a closed enumeration (a small,
//! fixed set of valid strings), the editor offers a dropdown instead of a free-form
//! text field. Keys not listed here stay free-form.
//!
//! The value sets are informed by plasma-nm's per-plugin option lists and the
//! NetworkManager VPN plugins' own key definitions (see `docs/ATTRIBUTION.md`); no
//! code was copied.

/// Return the allowed values for a known enum `key` under `service_type`, or `None`
/// when the key is free-form (the caller then renders a text field).
#[must_use]
pub fn enum_values(service_type: &str, key: &str) -> Option<&'static [&'static str]> {
    // Match on the plugin's short name so both the full D-Bus service type
    // (`org.freedesktop.NetworkManager.openvpn`) and a bare `openvpn` resolve.
    let plugin = service_type.rsplit('.').next().unwrap_or(service_type);
    match (plugin, key) {
        ("openvpn", "connection-type") => Some(&["tls", "password", "password-tls", "static-key"]),
        ("openvpn", "auth") => Some(&[
            "SHA1",
            "SHA224",
            "SHA256",
            "SHA384",
            "SHA512",
            "MD5",
            "RIPEMD160",
            "none",
        ]),
        ("openvpn", "key-direction") => Some(&["0", "1"]),
        ("openvpn", "remote-cert-tls") => Some(&["server", "client"]),
        ("openvpn", "tls-crypt-v2-force-cookie") => Some(&["yes", "no"]),
        ("openconnect", "protocol") => {
            Some(&["anyconnect", "nc", "gp", "pulse", "f5", "fortinet", "array"])
        }
        ("vpnc", "NAT Traversal Mode") => Some(&["natt", "none", "force-natt", "cisco-udp"]),
        ("vpnc", "Perfect Forward Secrecy") => Some(&["server", "nopfs", "dh1", "dh2", "dh5"]),
        ("vpnc", "IKE DH Group") => Some(&["dh1", "dh2", "dh5"]),
        ("vpnc", "Vendor") => Some(&["cisco", "netscreen"]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::enum_values;

    #[test]
    fn resolves_full_and_bare_service_types() {
        let full = enum_values("org.freedesktop.NetworkManager.openconnect", "protocol");
        let bare = enum_values("openconnect", "protocol");
        assert_eq!(full, bare);
        assert!(full.unwrap().contains(&"anyconnect"));
    }

    #[test]
    fn unknown_keys_are_freeform() {
        assert!(enum_values("openvpn", "remote").is_none());
        assert!(enum_values("nonexistent", "connection-type").is_none());
    }
}
