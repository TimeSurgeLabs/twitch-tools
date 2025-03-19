import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { readFile } from "@tauri-apps/plugin-fs";
import * as path from "@tauri-apps/api/path";
import "./App.css";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [textToSynthesize, setTextToSynthesize] = useState("");

  async function synthesizeSpeech() {
    setGreetMsg("Synthesizing audio...");
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const tempFileName: string = await invoke("synth_text", {
      text: textToSynthesize,
    });
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

  async function synthesizeAndPlayAudio() {
    setGreetMsg("Synthesizing and playing audio...");
    await invoke("synth_and_play_text", {
      text: textToSynthesize,
    });
  }

  return (
    <main className="container mx-auto p-4 max-w-2xl">
      <Card className="w-full">
        <CardHeader>
          <CardTitle className="text-2xl font-bold text-center">
            Welcome to Tauri + React
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="flex justify-center space-x-8">
            <a
              href="https://vitejs.dev"
              target="_blank"
              className="hover:opacity-80 transition-opacity"
            >
              <img src="/vite.svg" className="h-16 w-16" alt="Vite logo" />
            </a>
            <a
              href="https://tauri.app"
              target="_blank"
              className="hover:opacity-80 transition-opacity"
            >
              <img src="/tauri.svg" className="h-16 w-16" alt="Tauri logo" />
            </a>
            <a
              href="https://reactjs.org"
              target="_blank"
              className="hover:opacity-80 transition-opacity"
            >
              <img src={reactLogo} className="h-16 w-16" alt="React logo" />
            </a>
          </div>

          <p className="text-center text-muted-foreground">
            Click on the Tauri, Vite, and React logos to learn more.
          </p>

          <form
            className="space-y-4"
            onSubmit={(e) => {
              e.preventDefault();
              //synthesizeSpeech();
              synthesizeAndPlayAudio();
            }}
          >
            <div className="space-y-2">
              <Label htmlFor="text-input">Enter text to synthesize</Label>
              <Input
                id="text-input"
                value={textToSynthesize}
                onChange={(e) => setTextToSynthesize(e.currentTarget.value)}
                placeholder="Enter text to convert to speech..."
              />
            </div>
            <Button type="submit" className="w-full">
              Synthesize Speech
            </Button>
          </form>

          {greetMsg && (
            <p className="text-center text-sm text-muted-foreground">
              {greetMsg}
            </p>
          )}
        </CardContent>
      </Card>
    </main>
  );
}

export default App;
