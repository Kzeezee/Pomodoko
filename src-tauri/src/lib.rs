use serde_json::json;
use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_plugin_store::StoreExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migration = vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "CREATE TABLE tasks (id INTEGER PRIMARY KEY, name TEXT, completed BOOLEAN);",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new()
                .args(["--flag1", "--flag2"])
                .app_name("Pomodoko")
                .build())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:pomodoko.db", migration)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // This loads the store from disk
            let store = app.store("preferences.json")?;

            // Note that values must be serde_json::Value instances,
            // otherwise, they will not be compatible with the JavaScript bindings.
            // store.set("a".to_string(), json!("b"));
            if store.get("pomodoro".to_string()) == None {
                store.set("pomodoro".to_string(), json!(25 * 60));
            }
            if store.get("short_rest".to_string()) == None {
                store.set("short_rest".to_string(), json!(5 * 60));
            }
            if store.get("long_rest".to_string()) == None {
                store.set("long_rest".to_string(), json!(15 * 60));
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
