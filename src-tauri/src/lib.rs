mod fs;
mod http;

use fs::{mkdir, read, read_setting, remove, rename, touch, tree, write};
use http::do_request;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            do_request,
            tree,
            read_setting,
            read,
            write,
            remove,
            rename,
            mkdir,
            touch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
