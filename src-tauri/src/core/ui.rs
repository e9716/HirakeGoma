use std::collections::HashMap;
use std::sync::Mutex;

use crate::core::db::kv_store::SearchDatabase;
use kv::Json;
use tauri;
use tauri::App;
use tauri::Manager;
use tauri::State;
use tauri::Wry;

use crate::plugins::application_search;

use super::db::kv_store::SearchDatabaseItem;

#[tauri::command]
fn dbg_search_database_items(db: State<'_, SearchDatabase>) -> Result<(), String> {
    db.print_all_item();
    Ok(())
}

#[tauri::command]
fn get_all_search_database_items(
    db: State<'_, SearchDatabase>,
) -> HashMap<String, SearchDatabaseItem> {
    db.get_all_items()
}

#[tauri::command]
fn add_app_to_search_database(
    db: State<'_, SearchDatabase>,
    app_title: String,
    app_icon_path: String,
    app_path: String,
) -> Result<(), String> {
    let key = app_title.clone();
    let value = SearchDatabaseItem::newApplication(app_title, app_icon_path, app_path);
    let _ = db.insert(key, value);
    Ok(())
}

fn init_states(app: &mut App) {
    let search_database_state = SearchDatabase::init(false);
    app.manage(search_database_state);
}

fn init_window(app: &mut App<Wry>) {}

pub fn init_app() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            application_search::command::greet,
            dbg_search_database_items,
            add_app_to_search_database,
            get_all_search_database_items
        ])
        .setup(|app| {
            init_states(app);

            #[cfg(debug_assertions)]
            app.get_window("setting_window").unwrap().open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}