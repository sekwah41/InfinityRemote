// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod protocols;
mod devices;
mod utils;

mod commands;
use commands::CommandRegister;

fn main() {
    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![greet])
        .register_infinity_commands()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
