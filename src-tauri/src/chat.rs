use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
