// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Ordering;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ev_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn ev_number(user69:i32, secret_number:i32) -> String {

    // Generate secret number
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // Determine if user guess is right
    match user69.cmp(&secret_number) {
        Ordering::Less => return "your guess is too low".to_string(),
        Ordering::Greater => return "your number is too high".to_string(),
        Ordering::Equal => {
            return "u got the number :)".to_string();
        },
    };
}

