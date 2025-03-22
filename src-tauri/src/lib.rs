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

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Default, Debug)]
struct Config {
    twitch_username: String,
}

// Load config function using Tauri's config system
fn load_config(app: &tauri::AppHandle) -> Config {
    let config_path = app
        .path()
        .resolve("config.json", BaseDirectory::AppConfig)
        .unwrap_or_else(|_| {
            app.path()
                .resolve("config.json", BaseDirectory::AppLocalData)
                .unwrap()
        });

    if let Ok(contents) = fs::read_to_string(&config_path) {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Config::default()
    }
}

// Save config function using Tauri's config system
fn save_config(app: &tauri::AppHandle, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = app
        .path()
        .resolve("config.json", BaseDirectory::AppConfig)
        .unwrap_or_else(|_| {
            app.path()
                .resolve("config.json", BaseDirectory::AppLocalData)
                .unwrap()
        });

    println!("Saving config to: {}", config_path.display());

    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let contents = serde_json::to_string_pretty(config)?;
    fs::write(config_path, contents)?;
    Ok(())
}

// Tauri command
#[tauri::command]
fn set_twitch_username(app: tauri::AppHandle, username: String) -> Result<String, String> {
    let mut config = load_config(&app);
    config.twitch_username = username;
    save_config(&app, &config).map_err(|e| e.to_string())?;
    Ok("Username updated successfully".to_string())
}

#[tauri::command]
fn get_twitch_username(app: tauri::AppHandle) -> Result<String, String> {
    let config = load_config(&app);
    Ok(config.twitch_username)
}

#[tauri::command]
fn print_config(app: tauri::AppHandle) -> Result<String, String> {
    let config = load_config(&app);
    println!("Current config: {:?}", config);
    Ok("Config printed to console".to_string())
}

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

fn get_resources_dir(handle: tauri::AppHandle) -> PathBuf {
    let path = handle
        .path()
        .resolve("resources", BaseDirectory::Resource)
        .unwrap();
    // Remove \\?\ prefix if present
    let path_str = path.to_string_lossy();
    if path_str.starts_with(r"\\?\") {
        PathBuf::from(path_str.trim_start_matches(r"\\?\"))
    } else {
        path
    }
}

// This command sythesizes and plays text
#[tauri::command]
fn synth_and_play_text(text: &str, handle: tauri::AppHandle) -> Result<String, String> {
    // set PIPER_ESPEAKNG_DATA_DIRECTORY to the resources/espeak-ng folder
    let resources_dir = get_resources_dir(handle.clone());
    env::set_var(
        "PIPER_ESPEAKNG_DATA_DIRECTORY",
        resources_dir.to_string_lossy().to_string(),
    );

    // Read the variable back and log it to the console.
    match env::var("PIPER_ESPEAKNG_DATA_DIRECTORY") {
        Ok(val) => println!("PIPER_ESPEAKNG_DATA_DIRECTORY is set to: {}", val),
        Err(e) => println!("Error reading env variable: {}", e),
    }
    // return the environment variable

    // Ok::<String, String>(env::var("PIPER_ESPEAKNG_DATA_DIRECTORY").unwrap())

    println!("Synthesizing and playing text: {}", text);
    let text = text.to_string();
    // thread::spawn(move || {
    println!("Thread Synthesizing and playing text: {}", text);
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
    let audio =
        match app_state
            .synth
            .as_ref()
            .unwrap()
            .synthesize_parallel(text, None)
        {
            Ok(audio) => audio,
            Err(e) => return Ok(format!(
                "Error synthesizing speech, Is this application in the applications folder?: {}",
                e
            )),
        };
    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }

    // play the audio
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let buf = SamplesBuffer::new(1, 22050, samples);
    sink.append(buf);
    sink.sleep_until_end();
    println!("Thread finished synthesizing and playing");
    // });

    Ok("Did this crash?".to_string())
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
        .invoke_handler(tauri::generate_handler![
            synth_and_play_text,
            test_command,
            set_twitch_username,
            get_twitch_username,
            print_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
