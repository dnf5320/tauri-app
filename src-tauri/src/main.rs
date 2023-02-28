#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{Menu, MenuEntry, Submenu, MenuItem, CustomMenuItem};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        // 新增菜单
        .menu(Menu::with_items([
            MenuEntry::Submenu(Submenu::new(
            "File",
            Menu::with_items([
                MenuItem::CloseWindow.into(),
                #[cfg(target_os = "macos")]
                CustomMenuItem::new("hello", "Hello").into(),
            ]),
            )),
        ]))
        // 新增关闭提示的逻辑
        .on_window_event(|event|{
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } =>{
                    //阻止默认关闭
                    api.prevent_close();

                    let window = event.window().clone();
                    tauri::api::dialog::confirm(Some(&event.window()), "关闭应用", "确定关闭当前应用?", move| answer|{
                        if answer {
                            let _result =window.close();//直接接收一下即可，_表示让浏览器忽略未使用的变量
                        }
                    })
                },
                _ => {}//todo
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
