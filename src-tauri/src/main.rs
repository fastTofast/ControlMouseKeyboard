#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod op_mouse_keyboard;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            op_mouse_keyboard::move_mouse,
            op_mouse_keyboard::move_mouse2
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
