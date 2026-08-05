// SPDX-License-Identifier: MPL-2.0

//! NetworkManager VPN plugin "auth-dialog" helper support.
//!
//! Some VPNs (notably OpenConnect: Cisco AnyConnect, GlobalProtect, F5, Fortinet,
//! Juniper) authenticate interactively — a SAML/SSO web login, a password form, or
//! OTP — decided by the server at connect time. NetworkManager delegates this to the
//! plugin's external `auth-dialog` binary, which owns the whole login UX (it opens
//! its own browser window for SAML). We locate that binary from the plugin's `.name`
//! file, spawn it, feed it the connection's `vpn.data` + existing secrets in NM's exact
//! wire format, and read the resulting secrets from its stdout.
//!
//! See `docs/OPENCONNECT_SAML.md` for the protocol. Secret values are never logged.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

use super::BackendError;

/// VPN service-types routed through the plugin auth-dialog. OpenConnect's single
/// helper covers all its protocol variants (AnyConnect / GlobalProtect / F5 /
/// Fortinet / Juniper / Array) and auto-detects SAML vs password per server.
pub const AUTH_DIALOG_SERVICES: &[&str] = &["org.freedesktop.NetworkManager.openconnect"];

/// Whether this service-type should be handled via its auth-dialog helper rather than
/// the applet's built-in password field.
#[must_use]
pub fn uses_auth_dialog(service_type: &str) -> bool {
    AUTH_DIALOG_SERVICES.contains(&service_type)
}

/// Directories scanned for VPN plugin `.name` files, in priority order. Honors
/// `$NM_VPN_PLUGIN_DIR` (a single directory), else the common system locations.
fn plugin_dirs() -> Vec<PathBuf> {
    if let Ok(dir) = std::env::var("NM_VPN_PLUGIN_DIR")
        && !dir.is_empty()
    {
        return vec![PathBuf::from(dir)];
    }
    [
        "/run/current-system/sw/lib/NetworkManager/VPN",
        "/usr/lib/NetworkManager/VPN",
        "/usr/lib/x86_64-linux-gnu/NetworkManager/VPN",
        "/etc/NetworkManager/VPN",
    ]
    .iter()
    .map(PathBuf::from)
    .collect()
}

/// Parse a plugin `.name` file (INI/GKeyFile), extracting `[VPN Connection] service`
/// and `[GNOME] auth-dialog`. Other keys are ignored.
#[must_use]
pub fn parse_name_file(content: &str) -> (Option<String>, Option<PathBuf>) {
    let mut section = String::new();
    let mut service = None;
    let mut auth_dialog = None;

    for line in content.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = name.to_string();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match (section.as_str(), key.trim()) {
            ("VPN Connection", "service") => service = Some(value.trim().to_string()),
            ("GNOME", "auth-dialog") => auth_dialog = Some(PathBuf::from(value.trim())),
            _ => {}
        }
    }
    (service, auth_dialog)
}

/// Locate the auth-dialog binary for a service-type by scanning the plugin dirs for a
/// `.name` file whose service matches. Returns the first whose auth-dialog exists.
#[must_use]
pub fn find_auth_dialog(service_type: &str) -> Option<PathBuf> {
    for dir in plugin_dirs() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("name") {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&path) else {
                continue;
            };
            let (service, auth_dialog) = parse_name_file(&content);
            if service.as_deref() == Some(service_type)
                && let Some(binary) = auth_dialog
                && binary.exists()
            {
                return Some(binary);
            }
        }
    }
    None
}

/// An installed VPN plugin available for creating connections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpnPlugin {
    /// `vpn.service-type` D-Bus name.
    pub service_type: String,
    /// Human-visible plugin name.
    pub name: String,
}

/// Parse `[VPN Connection] service` and `name` from a `.name` file into a plugin entry.
#[must_use]
pub fn parse_plugin(content: &str) -> Option<VpnPlugin> {
    let mut section = String::new();
    let mut service = None;
    let mut name = None;
    for line in content.lines() {
        let line = line.trim();
        if let Some(s) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = s.to_string();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if section == "VPN Connection" {
            match key.trim() {
                "service" => service = Some(value.trim().to_string()),
                "name" => name = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }
    let service = service?;
    // Fall back to the service-type's last segment (e.g. `…openvpn` -> `openvpn`).
    let name = name.unwrap_or_else(|| {
        service
            .rsplit('.')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or(&service)
            .to_string()
    });
    Some(VpnPlugin {
        service_type: service,
        name,
    })
}

/// List installed VPN plugins (for the create picker), deduped by service-type and
/// sorted by name.
#[must_use]
pub fn list_plugins() -> Vec<VpnPlugin> {
    let mut plugins: Vec<VpnPlugin> = Vec::new();
    for dir in plugin_dirs() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("name") {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&path) else {
                continue;
            };
            if let Some(plugin) = parse_plugin(&content)
                && !plugins
                    .iter()
                    .any(|p| p.service_type == plugin.service_type)
            {
                plugins.push(plugin);
            }
        }
    }
    plugins.sort_by_key(|plugin| plugin.name.to_lowercase());
    plugins
}

/// Build the auth-dialog argv. OpenConnect's getopt accepts only these options, so we
/// pass no others (e.g. no `--external-ui-mode`).
#[must_use]
pub fn build_argv(
    uuid: &str,
    id: &str,
    service_type: &str,
    allow_interaction: bool,
    request_new: bool,
) -> Vec<String> {
    let mut argv = vec![
        "-u".to_string(),
        uuid.to_string(),
        "-n".to_string(),
        id.to_string(),
        "-s".to_string(),
        service_type.to_string(),
    ];
    if allow_interaction {
        argv.push("-i".to_string());
    }
    if request_new {
        argv.push("-r".to_string());
    }
    argv
}

/// Serialize the stdin payload in NM's exact wire format: `DATA_KEY`/`DATA_VAL` pairs,
/// then `SECRET_KEY`/`SECRET_VAL` pairs, then the `DONE`/`QUIT` terminator. Newlines
/// inside values are replaced with spaces.
#[must_use]
pub fn serialize_stdin(data: &[(String, String)], secrets: &[(String, String)]) -> String {
    fn sanitize(value: &str) -> String {
        value.replace(['\n', '\r'], " ")
    }

    let mut out = String::new();
    for (key, value) in data {
        out.push_str(&format!(
            "DATA_KEY={}\nDATA_VAL={}\n",
            sanitize(key),
            sanitize(value)
        ));
    }
    for (key, value) in secrets {
        out.push_str(&format!(
            "SECRET_KEY={}\nSECRET_VAL={}\n",
            sanitize(key),
            sanitize(value)
        ));
    }
    out.push_str("DONE\n\nQUIT\n\n");
    out
}

/// Parse the auth-dialog stdout: alternating `key\nvalue\n` lines, stopping at the
/// first empty key (the terminating blank line). Each pair becomes a `vpn.secrets`
/// entry.
#[must_use]
pub fn parse_stdout(output: &str) -> HashMap<String, String> {
    let mut lines = output.lines();
    let mut secrets = HashMap::new();
    while let Some(key) = lines.next() {
        if key.is_empty() {
            break;
        }
        let value = lines.next().unwrap_or("");
        secrets.insert(key.to_string(), value.to_string());
    }
    secrets
}

/// Spawn the auth-dialog helper, feed it `stdin_payload`, and return the secrets it
/// writes. The helper owns the interactive login UX (its own SAML browser window), so
/// this future may be pending for as long as the user takes to authenticate.
pub async fn run_auth_dialog(
    auth_dialog: &Path,
    argv: &[String],
    stdin_payload: &str,
) -> Result<HashMap<String, String>, BackendError> {
    let mut child = Command::new(auth_dialog)
        .args(argv)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| BackendError(format!("spawning auth-dialog: {e}")))?;

    // Payload is small (well under the pipe buffer), so writing it all before reading
    // stdout can't deadlock.
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(stdin_payload.as_bytes())
            .await
            .map_err(|e| BackendError(format!("writing auth-dialog stdin: {e}")))?;
    }

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)
            .await
            .map_err(|e| BackendError(format!("reading auth-dialog stdout: {e}")))?;
    }

    let status = child
        .wait()
        .await
        .map_err(|e| BackendError(format!("waiting for auth-dialog: {e}")))?;
    if !status.success() {
        return Err(BackendError(
            "auth-dialog exited without success".to_string(),
        ));
    }
    Ok(parse_stdout(&stdout))
}

/// Read a connection's `vpn.data` (`a{ss}`) via the Settings.Connection `GetSettings`
/// D-Bus method, as an ordered list of key/value pairs.
pub async fn fetch_vpn_data(connection_path: &str) -> Result<Vec<(String, String)>, BackendError> {
    let connection = zbus::Connection::system()
        .await
        .map_err(|e| BackendError(format!("connecting to the system bus: {e}")))?;
    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        connection_path,
        "org.freedesktop.NetworkManager.Settings.Connection",
    )
    .await
    .map_err(|e| BackendError(format!("opening connection proxy: {e}")))?;

    let settings: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>> = proxy
        .call("GetSettings", &())
        .await
        .map_err(|e| BackendError(format!("reading connection settings: {e}")))?;

    let Some(data_value) = settings.get("vpn").and_then(|vpn| vpn.get("data")) else {
        return Ok(Vec::new());
    };
    let owned = data_value
        .try_clone()
        .map_err(|e| BackendError(format!("decoding vpn.data: {e}")))?;
    let data: HashMap<String, String> = owned
        .try_into()
        .map_err(|e| BackendError(format!("decoding vpn.data: {e}")))?;
    Ok(data.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_name_file_service_and_auth_dialog() {
        let content = "[VPN Connection]\nname=openconnect\n\
                       service=org.freedesktop.NetworkManager.openconnect\n\
                       program=/x/nm-openconnect-service\n\
                       [GNOME]\nauth-dialog=/x/nm-openconnect-auth-dialog\n\
                       properties=/x/props\n";
        let (service, auth_dialog) = parse_name_file(content);
        assert_eq!(
            service.as_deref(),
            Some("org.freedesktop.NetworkManager.openconnect")
        );
        assert_eq!(
            auth_dialog,
            Some(PathBuf::from("/x/nm-openconnect-auth-dialog"))
        );
    }

    #[test]
    fn parses_plugin_service_and_name() {
        let content = "[VPN Connection]\nname=OpenVPN\n\
                       service=org.freedesktop.NetworkManager.openvpn\n[GNOME]\n";
        let plugin = parse_plugin(content).expect("plugin");
        assert_eq!(plugin.name, "OpenVPN");
        assert_eq!(
            plugin.service_type,
            "org.freedesktop.NetworkManager.openvpn"
        );
        // Falls back to the service suffix when no name is present.
        let plugin =
            parse_plugin("[VPN Connection]\nservice=org.freedesktop.NetworkManager.vpnc\n")
                .expect("plugin");
        assert_eq!(plugin.name, "vpnc");
    }

    #[test]
    fn routes_only_openconnect_by_default() {
        assert!(uses_auth_dialog(
            "org.freedesktop.NetworkManager.openconnect"
        ));
        assert!(!uses_auth_dialog("org.freedesktop.NetworkManager.openvpn"));
    }

    #[test]
    fn build_argv_adds_interaction_and_request_new_flags() {
        assert_eq!(
            build_argv("uuid", "id", "svc", false, false),
            vec!["-u", "uuid", "-n", "id", "-s", "svc"]
        );
        assert_eq!(
            build_argv("uuid", "id", "svc", true, true),
            vec!["-u", "uuid", "-n", "id", "-s", "svc", "-i", "-r"]
        );
    }

    #[test]
    fn serialize_stdin_matches_wire_format() {
        let data = vec![("gateway".to_string(), "vpn.example.com".to_string())];
        let secrets = vec![("cookie".to_string(), "old".to_string())];
        let out = serialize_stdin(&data, &secrets);
        assert_eq!(
            out,
            "DATA_KEY=gateway\nDATA_VAL=vpn.example.com\n\
             SECRET_KEY=cookie\nSECRET_VAL=old\n\
             DONE\n\nQUIT\n\n"
        );
    }

    #[test]
    fn serialize_stdin_replaces_newlines_in_values() {
        let data = vec![("k".to_string(), "a\nb".to_string())];
        let out = serialize_stdin(&data, &[]);
        assert!(out.starts_with("DATA_KEY=k\nDATA_VAL=a b\n"));
    }

    #[test]
    fn parse_stdout_reads_pairs_until_blank_line() {
        let out = "gateway\nvpn.example.com\ncookie\nabc123\n\ntrailing\nignored\n";
        let secrets = parse_stdout(out);
        assert_eq!(
            secrets.get("gateway").map(String::as_str),
            Some("vpn.example.com")
        );
        assert_eq!(secrets.get("cookie").map(String::as_str), Some("abc123"));
        assert_eq!(secrets.len(), 2); // stops at the blank line
    }

    #[tokio::test]
    async fn run_auth_dialog_spawns_and_parses_a_mock_helper() {
        // A mock helper: consume stdin, emit two secret pairs then a blank line.
        let script = std::env::temp_dir().join(format!("mock-authdlg-{}.sh", std::process::id()));
        std::fs::write(
            &script,
            "#!/bin/sh\ncat >/dev/null\nprintf 'gateway\\nvpn.example.com\\ncookie\\nabc123\\n\\n'\n",
        )
        .unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();

        let argv = build_argv("u", "n", "svc", true, false);
        let out = run_auth_dialog(&script, &argv, "DATA_KEY=a\nDATA_VAL=b\nDONE\n\nQUIT\n\n")
            .await
            .unwrap();
        std::fs::remove_file(&script).ok();

        assert_eq!(
            out.get("gateway").map(String::as_str),
            Some("vpn.example.com")
        );
        assert_eq!(out.get("cookie").map(String::as_str), Some("abc123"));
    }
}
