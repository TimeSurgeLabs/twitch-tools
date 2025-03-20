mod chat;

use lazy_static::lazy_static;
use piper_rs::synth::PiperSpeechSynthesizer;
use std::env;
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use tauri::path::BaseDirectory;
use tauri::Manager;

use rodio::buffer::SamplesBuffer;

struct AppState {
    synth: Option<PiperSpeechSynthesizer>,
    watched_username: String,
    stream_chat: bool,
}

lazy_static! {
    static ref APP_STATE: Mutex<AppState> = Mutex::new(AppState {
        synth: None,
        watched_username: String::new(),
        stream_chat: false,
    });
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

// This command sythesizes and plays text
#[tauri::command]
fn synth_and_play_text(text: &str, handle: tauri::AppHandle) -> Result<String, String> {
    let text = text.to_string();
    thread::spawn(move || {
        let mut app_state = APP_STATE.lock().unwrap();
        // if the synth is None, then we need to initialize it
        if app_state.synth.is_none() {
            // get the resources folder
            let resources_dir = get_resources_dir(handle);
            let config_path = Path::new(&resources_dir).join("model.onnx.json");
            let model = piper_rs::from_config_path(&config_path)
                .map_err(|e| e.to_string())
                .unwrap();
            model.set_speaker(50);
            let synth = PiperSpeechSynthesizer::new(model)
                .map_err(|e| e.to_string())
                .unwrap();
            app_state.synth = Some(synth);
        }

        // synthesize the text to speech
        let mut samples: Vec<f32> = Vec::new();
        let audio = app_state
            .synth
            .as_ref()
            .unwrap()
            .synthesize_parallel(text, None)
            .unwrap();
        for result in audio {
            samples.append(&mut result.unwrap().into_vec());
        }

        // play the audio
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let buf = SamplesBuffer::new(1, 22050, samples);
        sink.append(buf);
        sink.sleep_until_end();
    });

    Ok("Started processing".to_string())
}

#[tauri::command]
async fn test_command(handle: tauri::AppHandle) -> Result<String, String> {
    chat::test_function().await.map_err(|e| e.to_string())?;
    Ok("Chat connection successful".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![synth_and_play_text, test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
