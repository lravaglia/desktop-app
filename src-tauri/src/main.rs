// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lint_reasons)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    clippy::style,
    clippy::suspicious,
    warnings,
    unsafe_code,
    reason = "Make sure that the code adheres to best practices."
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    pretty_env_logger::try_init()?;
    color_eyre::install()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())?;

    Ok(())
}
