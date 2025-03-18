import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { readFile } from "@tauri-apps/plugin-fs";
import * as path from "@tauri-apps/api/path";
import { listen } from '@tauri-apps/api/event';
import "./App.css";

type DownloadStarted = {
  url: string;
  downloadId: number;
  contentLength: number;
};

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg("Synthesizing audio...");
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const tempFileName: string = await invoke("synth_text", { text: name });
    const fileContent = await readFile(tempFileName, {
      baseDir: path.BaseDirectory.Temp,
    });

    setGreetMsg("Playing audio...");
    const audio = new Audio(URL.createObjectURL(new Blob([fileContent])));
    audio
      .play()
      .catch((error) => {
        console.error("Error playing audio:", error);
      })
      .finally(() => {
        setGreetMsg("Audio playback complete");
        setTimeout(() => {
          setGreetMsg("");
        }, 1000);
      });
  }

  async function startDownload() {
    await invoke("download", { url: "https://example.com/file.mp3" });
  }
  
  listen<DownloadStarted>('download-progress', (event) => {
    console.log(
      `downloading ${event.payload.contentLength} bytes from ${event.payload.url}`
    );
    setName(`hello ${JSON.stringify(event.payload)}`);
  });

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <button onClick={startDownload}>Start Download</button>
      <p>{greetMsg}</p>
      <p>{name}</p>
    </main>
  );
}

export default App;
