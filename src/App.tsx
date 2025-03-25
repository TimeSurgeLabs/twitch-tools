import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

interface Speaker {
  id: number;
  name: string;
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [textToSynthesize, setTextToSynthesize] = useState("");
  const [connectedToTwitch, setConnectedToTwitch] = useState(false);
  const [speakers, setSpeakers] = useState<Speaker[]>([]);
  const [selectedSpeakerId, setSelectedSpeakerId] = useState<number | null>(50);

  // Function to fetch available speakers
  async function fetchSpeakers() {
    try {
      const speakerList = (await invoke("get_available_speakers")) as [
        number,
        string
      ][];
      const formattedSpeakers = speakerList.map(([id, name]) => ({ id, name }));
      // limit to the first 50
      setSpeakers(formattedSpeakers.slice(0, 50));
    } catch (error) {
      console.error("Failed to fetch speakers:", error);
      setGreetMsg("Error fetching available voices");
    }
  }

  // Function to set selected speaker
  async function handleSpeakerChange(speakerId: string) {
    try {
      setGreetMsg("Updating voice...");
      const id = parseInt(speakerId);
      await invoke("set_selected_speaker", { speakerId: id });
      setSelectedSpeakerId(id);
      setGreetMsg("Voice updated successfully");
    } catch (error) {
      console.error("Failed to set speaker:", error);
      setGreetMsg("Error setting voice");
    }
  }

  async function startTwitchChatReader() {
    const message = await invoke("start_twitch_chat_reader");
    setGreetMsg(message as string);
    setConnectedToTwitch(true);
  }

  // Function to synthesize and play audio
  async function synthesizeAndPlayAudio() {
    setGreetMsg("Synthesizing and playing audio...");
    const message = await invoke("synth_and_play_text", {
      text: textToSynthesize,
    });
    setGreetMsg(message as string);
  }

  async function killTwitchChatReader() {
    const message = await invoke("kill_twitch_chat_reader");
    setGreetMsg(message as string);
    setConnectedToTwitch(false);
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
      const username = (await invoke("get_twitch_username")) as string;
      setTwitchUsername(username);
      if (username) {
        setGreetMsg(`Current set Twitch username: ${username}`);
      } else {
        setGreetMsg("No Twitch username set. Please configure one.");
      }
    } catch (error) {
      console.error("Failed to fetch Twitch username:", error);
      setGreetMsg("Error fetching Twitch username");
    }
  }

  // Call the functions when component mounts
  useEffect(() => {
    fetchTwitchUsername();
    fetchSpeakers();
  }, []);

  return (
    <main className="container mx-auto p-4 max-w-2xl">
      <Card className="w-full">
        <CardHeader>
          <CardTitle className="text-2xl font-bold text-center">
            Twitch Tools
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <form
            className="space-y-4"
            onSubmit={(e) => {
              e.preventDefault();
              synthesizeAndPlayAudio();
            }}
          >
            <div className="space-y-2">
              <Label htmlFor="voice-select">Select Voice</Label>
              <Select
                onValueChange={handleSpeakerChange}
                value={selectedSpeakerId?.toString()}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select a voice" />
                </SelectTrigger>
                <SelectContent>
                  {speakers.map((speaker) => (
                    <SelectItem key={speaker.id} value={speaker.id.toString()}>
                      {speaker.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
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

          {!connectedToTwitch ? (
            <Button onClick={startTwitchChatReader} className="w-full">
              Start Chat Twitch Connection
            </Button>
          ) : (
            <Button onClick={killTwitchChatReader} className="w-full">
              Kill Chat Twitch Connection
            </Button>
          )}
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
