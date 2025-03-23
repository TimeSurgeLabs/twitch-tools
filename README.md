# Twitch Tools 
A group of tools to make twitch streaming easier, including local tts  
Built with Tauri + React + Typescript + Vite

## Install and Run
Have [tauri](https://v2.tauri.app/start/prerequisites/) installed  
Have [just](https://github.com/casey/just) installed  
Note: On windows you might also need LLVM `winget install llvm`  

```bash
npm install 
just dev
```

# Building
```bash
just build
```
Note: on mac, be sure to add a .env from env.example with your apple credentials. [Reference the tauri docs](https://v2.tauri.app/distribute/sign/macos/).

## Cross compiling to intel based macs  
Install rust intel support on the apple silicon mac, then build it 
```bash
rustup target add x86_64-apple-darwin  
just build-intel-mac  
```
## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
