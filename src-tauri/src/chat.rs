use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

const SERVER: &str = "irc.chat.twitch.tv";
const PORT: u16 = 6667;
const DEFAULT_NICKNAME: &str = "justinfan12345";

lazy_static! {
    static ref DISPLAY_NAME_REGEX: Regex = Regex::new(r"display-name=([^;]+)").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(r":([^!]+)!").unwrap();
    static ref MESSAGE_REGEX: Regex = Regex::new(r"PRIVMSG [^:]+:(.+)").unwrap();
}

pub async fn connect_to_twitch_chat(channel: &str, nickname: Option<&str>) -> Result<TcpStream> {
    // Connect to the Twitch IRC server
    let mut stream = TcpStream::connect((SERVER, PORT)).await?;

    // Send authentication info
    // For anonymous connection, we can use "justinfan" followed by any number
    stream.write_all(b"PASS SCHMOOPIIE\r\n").await?;
    stream
        .write_all(format!("NICK {}\r\n", nickname.unwrap_or(DEFAULT_NICKNAME)).as_bytes())
        .await?;
    stream
        .write_all(format!("JOIN #{}\r\n", channel.trim_start_matches('#')).as_bytes())
        .await?;

    // Request additional capabilities
    stream
        .write_all(b"CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership\r\n")
        .await?;

    // Flush the stream to ensure all commands are sent
    stream.flush().await?;

    // Print connection message
    println!(
        "Connected to #{} chat as anonymous viewer.",
        channel.trim_start_matches('#')
    );
    println!("Press Ctrl+C to exit");

    Ok(stream)
}

#[derive(Debug)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
}

pub fn parse_message(message: &str) -> Option<ChatMessage> {
    // Try to get display name first
    let username = if let Some(cap) = DISPLAY_NAME_REGEX.captures(message) {
        cap.get(1).map(|m| m.as_str().to_string())
    } else {
        // Fallback to username from IRC format
        USERNAME_REGEX
            .captures(message)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    };

    // Get message content
    let content = MESSAGE_REGEX
        .captures(message)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string());

    // Return Some only if we have both username and content
    match (username, content) {
        (Some(username), Some(content)) => Some(ChatMessage { username, content }),
        _ => None,
    }
}

pub async fn test_function(channel: &str) -> Result<()> {
    let stream = connect_to_twitch_chat(channel, None).await?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    println!("Starting to read messages...");

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("Connection closed by server");
                break;
            }
            Ok(_) => {
                // Handle PING messages to keep the connection alive
                if line.starts_with("PING") {
                    reader
                        .get_mut()
                        .write_all(b"PONG :tmi.twitch.tv\r\n")
                        .await?;
                    reader.get_mut().flush().await?;
                    continue;
                }

                if let Some(message) = parse_message(&line) {
                    println!("{}: {}", message.username, message.content);
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}

pub async fn start_twitch_chat_reader(
    channel: &str,
    tts_tx: &Sender<String>,
    kill_flag: &Arc<AtomicBool>,
) -> Result<()> {
    let stream = connect_to_twitch_chat(channel, None).await?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    println!("Starting to read messages...");

    loop {
        if kill_flag.load(Ordering::SeqCst) {
            println!("Kill signal received, stopping twitch chat reader...");
            break;
        }
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("Connection closed by server");
                break;
            }
            Ok(_) => {
                // Handle PING messages to keep the connection alive
                if line.starts_with("PING") {
                    reader.get_mut().write_all(b"PONG\r\n").await?;
                    reader.get_mut().flush().await?;
                    println!("PONG sent");
                    continue;
                }

                if kill_flag.load(Ordering::SeqCst) {
                    println!("Kill signal received, stopping twitch chat reader...");
                    break;
                }

                if let Some(message) = parse_message(&line) {
                    println!("{}: {}", message.username, message.content);
                    match tts_tx.send(format!(
                        "user {} said {}",
                        message.username, message.content
                    )) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Invalid tts_tx, another twitch_chat_reader is likely running, killing self");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}
