mod chat;
mod tts;

use lazy_static::lazy_static;
use piper_rs::synth::PiperSpeechSynthesizer;
use std::env;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::path::BaseDirectory;
use tauri::Manager;

use rodio::buffer::SamplesBuffer;

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Serialize, Deserialize, Default, Debug)]
struct Config {
    twitch_username: String,
    selected_speaker_id: i32,
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
    tts_tx: Option<Sender<String>>,
    tts_rx: Option<Receiver<String>>,
    kill_flag: Option<Arc<AtomicBool>>,
    audio_tx: Option<Sender<Vec<f32>>>,
    audio_rx: Option<Receiver<Vec<f32>>>,
}

lazy_static! {
    static ref APP_STATE: Mutex<AppState> = Mutex::new(AppState {
        synth: None,
        tts_tx: None,
        tts_rx: None,
        kill_flag: None,
        audio_tx: None,
        audio_rx: None,
    });
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

    println!("Synthesizing and playing text: {}", text);
    let text = text.to_string();
    println!("Thread Synthesizing and playing text: {}", text);
    let mut app_state = APP_STATE.lock().unwrap();
    // if the synth is None, then we need to initialize it
    if app_state.synth.is_none() {
        // get the resources folder
        let resources_dir = get_resources_dir(handle.clone());
        let config_path = Path::new(&resources_dir).join("model.onnx.json");
        let model = piper_rs::from_config_path(&config_path)
            .map_err(|e| e.to_string())
            .unwrap();

        // Get selected speaker from config
        let config = load_config(&handle);
        // set the speaker to the selected speaker
        model.set_speaker(config.selected_speaker_id as i64);

        let synth = PiperSpeechSynthesizer::new(model)
            .map_err(|e| e.to_string())
            .unwrap();
        app_state.synth = Some(synth);
    }

    // synthesize the text to speech
    let mut samples: Vec<f32> = Vec::new();
    let audio = match app_state
        .synth
        .as_ref()
        .unwrap()
        .synthesize_parallel(text, None)
    {
        Ok(audio) => audio,
        Err(e) => {
            return Ok(format!(
                "Error synthesizing speech, Is this application in the applications folder?: {}",
                e
            ))
        }
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

    Ok("Finished TTS!".to_string())
}

#[tauri::command]
async fn test_command(handle: tauri::AppHandle) -> Result<String, String> {
    let config = load_config(&handle);
    chat::test_function(&config.twitch_username)
        .await
        .map_err(|e| e.to_string())?;
    Ok("Chat connection successful".to_string())
}

#[tauri::command]
fn start_twitch_chat_reader(handle: tauri::AppHandle) -> Result<String, String> {
    let config = load_config(&handle);

    // Create a channel for communication
    let (tts_tx, tts_rx): (Sender<String>, Receiver<String>) = channel(); // Twitch -> TTS
    let tts_tx_clone = tts_tx.clone();
    let kill_flag = Arc::new(AtomicBool::new(false)); // NEW
    let kill_flag_clone = kill_flag.clone();

    {
        // set the tts_tx and tts_rx in the app state
        let mut app_state = APP_STATE.lock().unwrap();
        // Clear any existing kill flag and set the new one
        if let Some(existing_flag) = &app_state.kill_flag {
            existing_flag.store(true, Ordering::SeqCst); // Kill any existing reader
        }
        app_state.tts_tx = Some(tts_tx);
        app_state.tts_rx = Some(tts_rx);
        app_state.kill_flag = Some(kill_flag); // store for later kill
    };

    let channel_name = config.twitch_username.clone();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) =
                chat::start_twitch_chat_reader(&channel_name, &tts_tx_clone, &kill_flag_clone).await
            {
                eprintln!("Error in Twitch chat reader: {}", e);
            }
        });
    });

    // create tts->audio channel of Vec<f32>
    let (audio_tx, audio_rx): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = channel(); // TTS -> Audio
    {
        let mut app_state = APP_STATE.lock().unwrap();
        app_state.audio_tx = Some(audio_tx);
        app_state.audio_rx = Some(audio_rx);
    };

    // get vars for tts->audio thread
    let resources_dir = get_resources_dir(handle.clone());
    let handle_clone = handle.clone();
    let (tts_rx, audio_tx, kill_flag) = {
        let mut app_state = APP_STATE.lock().unwrap();
        (
            app_state.tts_rx.take().unwrap(),
            app_state.audio_tx.as_ref().unwrap().clone(),
            app_state.kill_flag.as_ref().unwrap().clone(),
        )
    };

    // create tts->audio thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            tts::synth_loop(tts_rx, &audio_tx, &kill_flag, &resources_dir, handle_clone)
                .await
                .unwrap();
        });
    });

    // get vars for audio->play thread
    let (audio_rx, kill_flag) = {
        let mut app_state = APP_STATE.lock().unwrap();
        (
            app_state.audio_rx.take().unwrap(),
            app_state.kill_flag.as_ref().unwrap().clone(),
        )
    };

    // create audio->play thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            tts::audio_loop(audio_rx, &kill_flag).await.unwrap();
        });
    });

    Ok("Twitch chat reader started".to_string())
}

#[tauri::command]
fn kill_twitch_chat_reader() -> Result<String, String> {
    println!("Killing twitch chat reader");
    println!("Attempting to acquire APP_STATE lock");
    let app_state = match APP_STATE.lock() {
        Ok(state) => {
            println!("Successfully acquired APP_STATE lock");
            state
        }
        Err(e) => {
            println!("Error acquiring APP_STATE lock: {}", e);
            return Err("Failed to acquire application state lock".to_string());
        }
    };
    println!("got here");
    if let Some(flag) = &app_state.kill_flag {
        println!("Setting kill flag");
        flag.store(true, Ordering::SeqCst); // Signal the thread to stop
        Ok("Twitch chat reader kill signal sent.".to_string())
    } else {
        println!("No chat reader running.");
        Err("No chat reader running.".to_string())
    }
}

#[tauri::command]
fn get_available_speakers(handle: tauri::AppHandle) -> Result<Vec<(i32, String)>, String> {
    let resources_dir = get_resources_dir(handle);
    let speakers = tts::get_available_speakers(&resources_dir).unwrap();
    Ok(speakers)
}

#[tauri::command]
async fn set_selected_speaker(app: tauri::AppHandle, speaker_id: i32) -> Result<String, String> {
    let mut config = load_config(&app);
    config.selected_speaker_id = speaker_id;
    save_config(&app, &config).map_err(|e| e.to_string())?;

    // Reinitialize the synthesizer with the new speaker in a background thread
    let resources_dir = get_resources_dir(app.clone());
    let config_path = Path::new(&resources_dir).join("model.onnx.json");
    let model = piper_rs::from_config_path(&config_path).map_err(|e| e.to_string())?;
    model.set_speaker(speaker_id as i64);

    let new_synth = PiperSpeechSynthesizer::new(model).map_err(|e| e.to_string())?;

    // Only lock APP_STATE for the brief moment we need to update the synthesizer
    let mut app_state = APP_STATE.lock().unwrap();
    app_state.synth = Some(new_synth);

    Ok("Speaker updated successfully".to_string())
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
            start_twitch_chat_reader,
            kill_twitch_chat_reader,
            get_available_speakers,
            set_selected_speaker,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
