// SPDX-License-Identifier: MPL-2.0

mod app;
mod backend;
mod config;
mod i18n;

use backend::VpnBackend;
use futures::StreamExt;

fn main() -> cosmic::iced::Result {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Headless debug paths, so the D-Bus layer can be exercised without the panel:
    //   --dump   print the VPN connections once and exit
    //   --watch  print them, then re-print on every NetworkManager change event
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--dump") {
        std::process::exit(run_dump());
    }
    if args.iter().any(|arg| arg == "--watch") {
        std::process::exit(run_watch());
    }

    // Starts the applet's event loop with `()` as the application's flags.
    cosmic::applet::run::<app::AppModel>(())
}

/// Print all VPN/WireGuard connections and their status to stdout. Returns a process
/// exit code (0 on success, 1 on error).
async fn print_connections() -> i32 {
    match backend::nm::NmBackend::new().list_connections().await {
        Ok(connections) => {
            if connections.is_empty() {
                println!("(no VPN connections)");
            }
            for connection in &connections {
                println!(
                    "{id}\n    uuid={uuid} type={type_label} status={status:?} \
                     autoconnect={autoconnect}\n    gateway={gateway} ipv4={ip4} ipv6={ip6}",
                    id = connection.id,
                    uuid = connection.uuid,
                    type_label = connection.type_label,
                    status = connection.status,
                    autoconnect = connection.autoconnect,
                    gateway = connection.gateway.as_deref().unwrap_or("-"),
                    ip4 = connection.ip4.as_deref().unwrap_or("-"),
                    ip6 = connection.ip6.as_deref().unwrap_or("-"),
                );
            }
            0
        }
        Err(error) => {
            eprintln!("error: {error}");
            1
        }
    }
}

/// One-shot dump.
fn run_dump() -> i32 {
    match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime.block_on(print_connections()),
        Err(error) => {
            eprintln!("failed to start async runtime: {error}");
            1
        }
    }
}

/// Dump, then re-dump on every NetworkManager change event (live-state smoke test).
fn run_watch() -> i32 {
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("failed to start async runtime: {error}");
            return 1;
        }
    };

    runtime.block_on(async {
        let _ = print_connections().await;
        match backend::nm::event_ticks().await {
            Ok(mut ticks) => {
                eprintln!("watching NetworkManager events (Ctrl-C to stop)…");
                while ticks.next().await.is_some() {
                    println!("--- NetworkManager change ---");
                    let _ = print_connections().await;
                }
                eprintln!("event stream ended (NetworkManager unavailable)");
                1
            }
            Err(error) => {
                eprintln!("error: {error}");
                1
            }
        }
    })
}
