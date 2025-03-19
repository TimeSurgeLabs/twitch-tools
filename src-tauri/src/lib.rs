mod chat;

use crate::chat::connect_to_twitch_chat;
use lazy_static::lazy_static;
use piper_rs::synth::PiperSpeechSynthesizer;
use std::env;
use std::path::Path;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::Manager;
use uuid::Uuid;

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

#[tauri::command]
fn synth_and_play_text(text: &str, handle: tauri::AppHandle) -> Result<String, String> {
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
    
    // synthesize the text to speech
    let mut samples: Vec<f32> = Vec::new();
    let audio = app_state.synth.as_ref().unwrap().synthesize_parallel(text.to_string(), None).unwrap();
    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }

    // play the audio
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let buf = SamplesBuffer::new(1, 22050, samples);
    sink.append(buf);
    sink.sleep_until_end();

    Ok("Complete".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![synth_text, synth_and_play_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
