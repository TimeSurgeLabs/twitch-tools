use anyhow::Result;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
use std::env;
use std::path::Path;
// use rodio::SamplesBuffer;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub async fn synth_loop(
    tts_rx: Receiver<String>,
    audio_tx: &Sender<Vec<f32>>,
    kill_flag: &Arc<AtomicBool>,
    resources_dir: &PathBuf,
) -> Result<()> {
    println!("Starting synth loop");
    // set PIPER_ESPEAKNG_DATA_DIRECTORY to the resources folder
    env::set_var(
        "PIPER_ESPEAKNG_DATA_DIRECTORY",
        resources_dir.to_string_lossy().to_string(),
    );
    println!(
        "PIPER_ESPEAKNG_DATA_DIRECTORY set to {}",
        resources_dir.to_string_lossy().to_string()
    );
    // get the resources folder
    let config_path = Path::new(&resources_dir).join("model.onnx.json");
    let model = piper_rs::from_config_path(&config_path)
        .map_err(|e| e.to_string())
        .unwrap();
    model.set_speaker(50);
    let synth = PiperSpeechSynthesizer::new(model)
        .map_err(|e| e.to_string())
        .unwrap();
    println!("tts model initialized");
    loop {
        if kill_flag.load(Ordering::SeqCst) {
            println!("Kill signal received, stopping synthesizer loop...");
            break;
        }
        let text = tts_rx.recv().unwrap();
        println!("Synthesizing: {}", text);
        let text = text.to_string();

        // synthesize the text to speech
        let mut samples: Vec<f32> = Vec::new();
        let audio = match synth.synthesize_parallel(text, None) {
            Ok(audio) => {
                println!("Successfully synthesized audio");
                audio
            }
            Err(e) => {
                println!("Error synthesizing: {}", e);
                break;
            }
        };
        for result in audio {
            samples.append(&mut result.unwrap().into_vec());
            println!("samples: {:?}", samples.len());
        }
        if kill_flag.load(Ordering::SeqCst) {
            println!("Kill signal received, stopping synthesizer loop...");
            break;
        }
        println!("Sending audio to audio_tx");

        audio_tx.send(samples).unwrap();
    }

    Ok(())
}

pub async fn audio_loop(audio_rx: Receiver<Vec<f32>>, kill_flag: &Arc<AtomicBool>) -> Result<()> {
    println!("Starting audio loop");
    loop {
        if kill_flag.load(Ordering::SeqCst) {
            println!("Kill signal received, stopping audio loop...");
            break;
        }

        let samples = audio_rx.recv().unwrap();

        // play the audio
        println!("Playing audio");
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let buf = SamplesBuffer::new(1, 22050, samples);
        sink.append(buf);

        while !sink.empty() {
            if kill_flag.load(Ordering::SeqCst) {
                sink.stop();
                println!("Kill signal received, stopping audio loop...");
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        println!("Thread finished synthesizing and playing");
    }
    Ok(())
}
