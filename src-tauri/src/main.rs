// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window;

use tauri::{Manager, SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem, SystemTrayMenuItem};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler!(first_tauri, window::load, window::app_add_event))
        .system_tray(SystemTray::new()
            .with_menu(SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("showMain", "展示主界面"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("quit", "退出"))
            ))
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::DoubleClick {
                    tray_id: _,
                    position: _,
                    size: _, ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let x = window.is_visible().unwrap();
                    println!("{:?}", x);
                    let result = window.show();
                    match result {
                        Ok(_) => { println!("sys double click ok!") }
                        Err(_) => { println!("sys double click err!") }
                    }
                }
                SystemTrayEvent::MenuItemClick{id,..}=>{
                    match id.as_str() {
                        "quit"=>{
                            std::process::exit(0);
                        }
                        "showMain"=>{
                            let window = app.get_window("main").unwrap();
                            window.show().unwrap();
                        }
                        &_ => {}
                    }
                }
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn first_tauri(str: &str) -> String {
    println!("execute....");
    format!("First tauri! Fisrt {}", str)
}
