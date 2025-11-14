// Tauri setup for desktop wrapper around Leptos SSR application
// Starts the Leptos server and wraps it in a native window

use std::process::Child;
#[cfg(not(dev))]
use std::process::Command;
use std::sync::Mutex;

use tauri::Manager;

struct ServerProcess(Mutex<Option<Child>>);

// Mobile entry point - required for Android/iOS
#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn mobile_main() {
    run();
}

fn create_window<R: tauri::Runtime>(app: &impl tauri::Manager<R>, url: &str) -> tauri::Result<tauri::WebviewWindow<R>> {
    #[allow(unused_mut)]
    let mut builder = tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::External(url.parse().unwrap()));

    // Desktop-only configuration
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder
            .title("Airbnb Clone")
            .inner_size(1200.0, 800.0)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true);
    }

    builder.build()
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Mobile (Android/iOS): Connect to dev server via port forwarding
            #[cfg(any(target_os = "android", target_os = "ios"))]
            {
                app.manage(ServerProcess(Mutex::new(None)));
                create_window(app, "http://localhost:3000")?;
            }

            // Desktop production: start the bundled Leptos server
            #[cfg(all(not(dev), not(any(target_os = "android", target_os = "ios"))))]
            {
                let resource_dir = app.path().resource_dir().expect("failed to get resource dir");

                let server_binary = resource_dir
                    .join("_up_")
                    .join("target")
                    .join("aarch64-apple-darwin")
                    .join("release")
                    .join("server");
                let site_root = resource_dir.join("_up_").join("target").join("site");

                let server_process = Command::new(&server_binary)
                    .env("LEPTOS_SITE_ROOT", site_root.to_str().unwrap())
                    .env("LEPTOS_SITE_ADDR", "127.0.0.1:3000")
                    .spawn()
                    .expect("failed to start server");

                app.manage(ServerProcess(Mutex::new(Some(server_process))));

                // Give server time to start
                std::thread::sleep(std::time::Duration::from_millis(500));

                create_window(app, "http://127.0.0.1:3000")?;
            }

            // Desktop dev mode: server already running via cargo leptos watch
            #[cfg(all(dev, not(any(target_os = "android", target_os = "ios"))))]
            {
                app.manage(ServerProcess(Mutex::new(None)));
                create_window(app, "http://localhost:3000")?;
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                // Kill server when window closes (production only)
                if let Some(state) = window.try_state::<ServerProcess>() {
                    if let Ok(mut process) = state.0.lock() {
                        if let Some(mut child) = process.take() {
                            let _ = child.kill();
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
