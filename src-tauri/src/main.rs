// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod page;
mod game;

use page::Response;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![game_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn game_loop(state: serde_json::Value) -> Result<Response, String> {
    // let mut response;
    if let Some(state) = state.as_object() {
        if let Some(returning_element) = state["element_id"].as_str() {
            if let Some(returned_data) = state["returned_data"].as_str() {
                println!("Element ID: {}", returning_element);
                println!("Data: {}", returned_data);
            } else {
                // handle nothing returned
            }
        };
    };

    game::game().map_err(|_| "Something went wrong with the game.".to_string())
}


