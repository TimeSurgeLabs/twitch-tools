import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [textToSynthesize, setTextToSynthesize] = useState("");

  async function testCommand() {
    const message = await invoke("test_command");
    setGreetMsg(message as string);
  }

  // Function to synthesize and play audio
  async function synthesizeAndPlayAudio() {
    setGreetMsg("Synthesizing and playing audio...");
    const message = await invoke("synth_and_play_text", {
      text: textToSynthesize,
    });
    setGreetMsg(message as string);
  }

  // Function to set the Twitch username to Tauri
  async function setTwitchUsernameToTauri(username: string) {
    const message = await invoke("set_twitch_username", {
      username: username,
    });
    setGreetMsg(message as string);
  }

  // Effect to get Twitch username on component mount
  const [twitchUsername, setTwitchUsername] = useState("Twitch Username Here");
  
  // Function to fetch the Twitch username from the backend
  async function fetchTwitchUsername() {
    try {
      const username = await invoke("get_twitch_username") as string;
      setTwitchUsername(username);
      if (username) {
        setGreetMsg(`Connected to Twitch username: ${username}`);
      } else {
        setGreetMsg("No Twitch username set. Please configure one.");
      }
    } catch (error) {
      console.error("Failed to fetch Twitch username:", error);
      setGreetMsg("Error fetching Twitch username");
    }
  }
  
  // Call the function when component mounts
  useEffect(() => {
    fetchTwitchUsername();
  }, []);

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

          <Button onClick={testCommand} className="w-full">
            Start Chat Twitch Connection
          </Button>

          <div className="space-y-2 mt-4">
            <Label htmlFor="twitch-username">Twitch Username</Label>
            <div className="flex gap-2">
              <Input
                id="twitch-username"
                value={twitchUsername}
                onChange={(e) => setTwitchUsername(e.currentTarget.value)}
                placeholder="Enter your Twitch username..."
              />
              <Button onClick={() => setTwitchUsernameToTauri(twitchUsername)}>
                Save
              </Button>
            </div>
          </div>
          <p>Twitch Username: {twitchUsername}</p>

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
