#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  AppHandle,
  CustomMenuItem,
  GlobalShortcutManager,
  Manager,
  SystemTray,
  SystemTrayEvent,
  SystemTrayMenu,
};

fn main() {
  let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("settings", "Settings"))
    .add_item(CustomMenuItem::new("update", "Check for Updates"))
    .add_item(CustomMenuItem::new("quit", "Quit"));

  let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle();
      register_visibility_shortcut(app_handle)?;
      Ok(())
    })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| {
      if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        println!("System tray menu clicked: {}", id);
        if id.as_ref() == "quit" {
          app.exit(0);
        }
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn register_visibility_shortcut(app_handle: AppHandle) -> tauri::Result<()> {
  let mut shortcut_manager = app_handle.global_shortcut();
  let handle = app_handle.clone();
  shortcut_manager.register("CmdOrCtrl+Shift+O", move || {
    if let Some(window) = handle.get_window("main") {
      match window.is_visible() {
        Ok(true) => {
          if let Err(err) = window.hide() {
            eprintln!("Failed to hide window: {}", err);
          }
        }
        Ok(false) => {
          if let Err(err) = window.show() {
            eprintln!("Failed to show window: {}", err);
          }
          if let Err(err) = window.set_focus() {
            eprintln!("Failed to focus window: {}", err);
          }
        }
        Err(err) => {
          eprintln!("Failed to determine window visibility: {}", err);
        }
      }
    }
  })?;
  Ok(())
}
