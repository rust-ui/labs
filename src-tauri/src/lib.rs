// Tauri setup for desktop wrapper around Leptos SSR application

// Mobile entry point - required for Android/iOS
#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn mobile_main() {
    run();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
