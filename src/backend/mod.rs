// SPDX-License-Identifier: MPL-2.0

//! VPN backend abstraction.
//!
//! All NetworkManager D-Bus access sits behind the [`VpnBackend`] trait so the UI
//! layer depends only on our own [`model`] types and can be driven by a
//! [`fake::FakeBackend`] in tests. The live implementation is [`nm::NmBackend`].

use std::fmt;

pub mod enums;
pub mod model;
pub mod nm;
pub mod vpn_auth;

#[cfg(test)]
pub mod fake;

pub use model::VpnConnection;

/// Error surfaced by a backend operation. Wraps the underlying cause as a message
/// so callers (and the UI) never depend on the D-Bus crate's error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendError(pub String);

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for BackendError {}

/// Read/query and activate/deactivate VPN connections. Further mutations (delete,
/// import, secrets) are added in later milestones.
pub trait VpnBackend {
    /// List all saved VPN + WireGuard connections with their current status,
    /// ordered for display (active first, then by name).
    fn list_connections(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<VpnConnection>, BackendError>> + Send;

    /// Activate a saved connection by UUID. NetworkManager routes VPN vs WireGuard
    /// internally, so callers don't branch on kind.
    fn activate(
        &self,
        uuid: &str,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Deactivate the active connection for this UUID.
    fn deactivate(
        &self,
        uuid: &str,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Set whether NetworkManager may auto-activate this profile.
    fn set_autoconnect(
        &self,
        uuid: &str,
        on: bool,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Delete ("forget") a saved connection profile.
    fn forget(
        &self,
        uuid: &str,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Import a VPN config file (`.ovpn` OpenVPN, or a WireGuard `.conf`) into
    /// NetworkManager, creating a connection.
    fn import(
        &self,
        path: &std::path::Path,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Create a new plugin (`type=vpn`) connection from a spec.
    fn create_vpn(
        &self,
        spec: model::NewVpn,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Create a new WireGuard connection from a spec.
    fn create_wireguard(
        &self,
        spec: model::NewWireGuard,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Read a saved WireGuard tunnel's editable fields, for prefilling the editor.
    fn read_wireguard(
        &self,
        uuid: &str,
    ) -> impl std::future::Future<Output = Result<model::WgEdit, BackendError>> + Send;

    /// Update a saved WireGuard tunnel (interface + peers).
    fn update_wireguard(
        &self,
        uuid: &str,
        spec: model::NewWireGuard,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Read a saved plugin VPN's editable fields, for prefilling the editor.
    fn read_vpn(
        &self,
        uuid: &str,
    ) -> impl std::future::Future<Output = Result<model::VpnEdit, BackendError>> + Send;

    /// Update a saved plugin VPN's general settings and `vpn.data`.
    fn update_vpn(
        &self,
        uuid: &str,
        spec: model::NewVpn,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;

    /// Duplicate a saved connection (any type) as `<name> (copy)`.
    fn duplicate(
        &self,
        uuid: &str,
    ) -> impl std::future::Future<Output = Result<(), BackendError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::VpnBackend;
    use super::fake::FakeBackend;
    use super::model::{VpnConnection, VpnKind, VpnStatus};

    fn sample() -> VpnConnection {
        VpnConnection {
            uuid: "u1".into(),
            id: "Work".into(),
            kind: VpnKind::Plugin,
            status: VpnStatus::Connected,
            service_type: Some("org.freedesktop.NetworkManager.openvpn".into()),
            last_used: 0,
            type_label: "OpenVPN".into(),
            gateway: Some("vpn.example.com".into()),
            ip4: Some("10.0.0.2/24".into()),
            ip6: None,
            dns_servers: Vec::new(),
            rx_bytes: 0,
            tx_bytes: 0,
            autoconnect: true,
        }
    }

    #[tokio::test]
    async fn fake_backend_returns_connections() {
        let backend = FakeBackend::with(vec![sample()]);
        let out = backend.list_connections().await.expect("ok");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "Work");
    }

    #[tokio::test]
    async fn fake_backend_can_fail() {
        let backend = FakeBackend::failing("boom");
        let err = backend.list_connections().await.expect_err("should fail");
        assert_eq!(err.to_string(), "boom");
    }
}
