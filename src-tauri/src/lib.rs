use lazy_static::lazy_static;
use piper_rs::synth::PiperSpeechSynthesizer;
use std::env;
use std::path::Path;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::Manager;
use uuid::Uuid;

use tauri::{AppHandle, Emitter};

#[tauri::command]
fn download(app: AppHandle, url: String) {
    app.emit("download-started", &url).unwrap();
    // Clone the app handle outside the loop
    let app_clone = app.clone();

    // Spawn a single thread to handle all progress updates
    std::thread::spawn(move || {
        for progress in [1, 15, 50, 80, 100] {
            std::thread::sleep(std::time::Duration::from_secs(1));
            app_clone.emit("download-progress", progress).unwrap();
        }
        app_clone.emit("download-finished", &url).unwrap();
    });
}

struct AppState {
    synth: Option<PiperSpeechSynthesizer>,
}

lazy_static! {
    static ref APP_STATE: Mutex<AppState> = Mutex::new(AppState { synth: None });
}

fn get_temp_dir() -> String {
    env::temp_dir().to_string_lossy().to_string()
}

fn get_resources_dir(handle: tauri::AppHandle) -> String {
    handle
        .path()
        .resolve("resources", BaseDirectory::Resource)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

// this command will synthesize text to speech and output the path to the new temp file
// the temp file will be deleted when the app closes
// the command will return the path to the temp file
#[tauri::command]
fn synth_text(text: &str, handle: tauri::AppHandle) -> Result<String, String> {
    let mut app_state = APP_STATE.lock().unwrap();
    // if the synth is None, then we need to initialize it
    if app_state.synth.is_none() {
        // get the resources folder
        let resources_dir = get_resources_dir(handle);
        let config_path = Path::new(&resources_dir).join("model.onnx.json");
        let model = piper_rs::from_config_path(&config_path).map_err(|e| e.to_string())?;
        model.set_speaker(50);
        let synth = PiperSpeechSynthesizer::new(model).map_err(|e| e.to_string())?;
        app_state.synth = Some(synth);
    }
    let id = Uuid::new_v4();
    // generate a new temp file name. Should be a {{uuid}}.wav
    let temp_file_name = format!("{}.wav", id.to_string());
    let temp_file_path = Path::new(&get_temp_dir()).join(temp_file_name.clone());

    // synthesize the text to speech
    app_state
        .synth
        .as_ref()
        .unwrap()
        .synthesize_to_file(Path::new(&temp_file_path), text.to_string(), None)
        .map_err(|e| e.to_string())?;

    Ok(temp_file_name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![synth_text, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
