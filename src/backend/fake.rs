// SPDX-License-Identifier: MPL-2.0

//! In-memory [`VpnBackend`] for tests — lets us exercise consumers of the backend
//! without a live D-Bus / NetworkManager.

use super::model::VpnConnection;
use super::{BackendError, VpnBackend};

/// A fake backend returning a fixed list (or a canned error).
#[derive(Debug, Default, Clone)]
pub struct FakeBackend {
    /// Connections to return from `list_connections`.
    pub connections: Vec<VpnConnection>,
    /// When set, `list_connections` fails with this message instead.
    pub error: Option<String>,
    /// When set, `activate`/`deactivate` fail with this message.
    pub op_error: Option<String>,
}

impl FakeBackend {
    /// Backend that returns the given connections.
    pub fn with(connections: Vec<VpnConnection>) -> Self {
        Self {
            connections,
            ..Default::default()
        }
    }

    /// Backend whose `list_connections` always fails.
    pub fn failing(message: &str) -> Self {
        Self {
            error: Some(message.to_string()),
            ..Default::default()
        }
    }
}

impl VpnBackend for FakeBackend {
    async fn list_connections(&self) -> Result<Vec<VpnConnection>, BackendError> {
        match &self.error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(self.connections.clone()),
        }
    }

    async fn activate(&self, _uuid: &str) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn deactivate(&self, _uuid: &str) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn set_autoconnect(&self, _uuid: &str, _on: bool) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn forget(&self, _uuid: &str) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn import(&self, _path: &std::path::Path) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn create_vpn(&self, _spec: super::model::NewVpn) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn create_wireguard(
        &self,
        _spec: super::model::NewWireGuard,
    ) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn read_vpn(&self, _uuid: &str) -> Result<super::model::VpnEdit, BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(super::model::VpnEdit::default()),
        }
    }

    async fn read_wireguard(&self, _uuid: &str) -> Result<super::model::WgEdit, BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(super::model::WgEdit::default()),
        }
    }

    async fn update_wireguard(
        &self,
        _uuid: &str,
        _spec: super::model::NewWireGuard,
    ) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn update_vpn(
        &self,
        _uuid: &str,
        _spec: super::model::NewVpn,
    ) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }

    async fn duplicate(&self, _uuid: &str) -> Result<(), BackendError> {
        match &self.op_error {
            Some(message) => Err(BackendError(message.clone())),
            None => Ok(()),
        }
    }
}
