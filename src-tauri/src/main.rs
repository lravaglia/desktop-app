// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// These harsh lints ensure that the project adheres to best practices.
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
)]

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    pretty_env_logger::try_init()?;
    color_eyre::install()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())?;

    Ok(())
}
