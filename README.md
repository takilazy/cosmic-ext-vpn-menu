# COSMIC VPN Menu

A [COSMIC™](https://system76.com/cosmic) panel applet for managing VPN connections
via NetworkManager.

It is deliberately **VPN-only**: it manages VPN and WireGuard connections and
implements no Wi-Fi, wired, cellular, or hotspot features — those stay with the stock
`cosmic-applet-network`. It replicates the VPN functionality of KDE's plasma-nm.

> Screenshots: _TODO — add panel icon + popup, detail panel, and editor screenshots._

## Features

- **List & status** — all saved VPN and WireGuard connections with live status; the
  panel icon reflects connected / connecting / disconnected.
- **Connect / disconnect** — one click, with optimistic UI and reconciliation.
- **Live updates** — driven by NetworkManager signals (no polling); survives NM
  restart and D-Bus loss.
- **Per-connection detail** — type, gateway, IPv4/IPv6, connected-for duration, an
  autoconnect toggle, and forget-with-confirmation.
- **Secrets** — the applet registers as a NetworkManager secret agent and prompts for
  passwords/OTP in the popup. Secrets are never logged or written to disk by the
  applet.
- **OpenConnect SAML/SSO** — for OpenConnect (Cisco AnyConnect, GlobalProtect, F5,
  Fortinet, Juniper), the plugin's `auth-dialog` helper is launched, which runs the
  interactive login (a SAML browser window, a password form, or OTP — auto-detected
  by the server) and returns the session cookie.
- **Import** — `.ovpn` (OpenVPN) and WireGuard `.conf`, via an XDG portal file picker.
- **Create & edit** — create a connection of any installed plugin type (a generic
  editor: type picker + `vpn.data` fields) or a WireGuard tunnel (interface + peer);
  edit an existing plugin VPN's name, autoconnect, and data.

## Requirements

- NetworkManager (runtime), reachable on the system D-Bus.
- The relevant NetworkManager VPN plugin(s) installed for the protocols you use
  (e.g. `NetworkManager-openvpn`, `NetworkManager-openconnect`, `NetworkManager-vpnc`,
  …). WireGuard needs no plugin.
- A COSMIC session to host the applet.

## Build & run

This project targets NixOS; both a `flake.nix` and a `shell.nix` are provided.

With flakes:

```sh
nix develop               # enter the dev shell
nix build                 # build the package (binary + .desktop + metainfo + icon)
nix run                   # build and run for testing
```

Or with the classic dev shell + justfile:

```sh
nix-shell                 # enter the dev shell (rust, pkg-config, wayland, … + just)
just run                  # build (release) and run for testing
just build-release        # build only
sudo just install         # install binary, .desktop, metainfo, and icon
```

On other distributions, install the usual libcosmic build deps (`pkg-config`,
`wayland`, `libxkbcommon`, `vulkan-loader`, `libGL`, `fontconfig`, `freetype`,
`expat`, plus `just`) and use the same `just` recipes.

After installing, add **COSMIC VPN Menu** to the panel via COSMIC Settings →
Desktop → Panel → applets.

### Headless debugging (no panel needed)

```sh
cargo run -- --dump    # print all VPN/WireGuard connections and their status, then exit
cargo run -- --watch   # print them, then re-print on every NetworkManager change event
```

## Packaging (NixOS)

Vendor dependencies and build offline for a reproducible package:

```sh
just vendor
just build-vendored
just rootdir=$out prefix=/usr install
```

Runtime needs NetworkManager and, at build time, the libcosmic dependencies listed
above (see `shell.nix` for the exact set). The applet discovers VPN plugin
`auth-dialog` helpers from the plugin `.name` files; honor `$NM_VPN_PLUGIN_DIR` if
your packaging places them non-standardly.

## Translations

Localization uses [Fluent](https://projectfluent.org/) under
[`i18n/`](./i18n). English (`i18n/en`) is the source of truth; any missing locale or
message falls back to English automatically. To add or improve a translation, copy
`i18n/en/cosmic_ext_vpn_menu.ftl` to `i18n/<locale>/cosmic_ext_vpn_menu.ftl` and
translate the values, keeping the message ids and `{ $placeholders }` intact.
Machine-seeded translations should be reviewed by native speakers.

## Architecture

All NetworkManager D-Bus access sits behind the `VpnBackend` trait
(`src/backend/`), so the UI depends only on our own model types and can be driven
by a fake backend in tests. The live implementation (`nm.rs`) is the only module
that touches the `nmrs` bindings. The applet keeps long-lived watchers — NM state
signals, the secret agent, VPN failure reasons, and logind suspend/resume — as
`iced` subscriptions that feed messages into the update loop. WireGuard is handled
as a native device type distinct from plugin VPNs, and OpenConnect interactive/SAML
auth is delegated to NM's `nm-openconnect-auth-dialog` helper.

## License

MPL-2.0. See [LICENSE](./LICENSE). This project was written from behavioral study of
GPL references (KDE plasma-nm, `cosmic-applets`) without copying their code; it depends
on the MPL-2.0 `libcosmic` and the MIT-licensed `nmrs` NetworkManager bindings.
