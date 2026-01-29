import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

import Header from "./components/Header";
import Sidebar from "./components/Sidebar";
import Console from "./components/Console";
import SettingsModal from "./components/SettingsModal";

interface Config {
  port: number;
  pin: string;
  key_bindings: Record<string, string>;
}

function App() {
  const [serverRunning, setServerRunning] = useState(false);
  const [qrCode, setQrCode] = useState("");
  const [clientCount, setClientCount] = useState(0);
  const [config, setConfig] = useState<Config | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [localIp, setLocalIp] = useState("");

  useEffect(() => {
    // Initialize
    invoke<string>("get_local_ip").then(setLocalIp);
    invoke<Config>("get_config").then(setConfig);

    // Listeners
    const unlistenStatus = listen("server-status", (event) => {
      setServerRunning(event.payload === "running");
    });

    const unlistenClients = listen<number>("client-count", (event) => {
      setClientCount(event.payload);
    });

    return () => {
      unlistenStatus.then((fn) => fn());
      unlistenClients.then((fn) => fn());
    };
  }, []);

  const generateQr = async (ip: string, port: number, pin: string) => {
    try {
      const qr = await invoke<string>("generate_qr_code", {
        data: `mobilehero://${ip}:${port}?pin=${pin}`,
      });
      setQrCode(qr);
    } catch (error) {
      console.error("QR Error:", error);
    }
  };

  const startServer = async () => {
    try {
      if (config) {
        await invoke("start_server");
        await generateQr(localIp, config.port, config.pin);
      }
    } catch (error) {
      console.error("Start Error:", error);
    }
  };

  const stopServer = async () => {
    try {
      await invoke("stop_server");
      setQrCode("");
      setClientCount(0);
    } catch (error) {
      console.error("Stop Error:", error);
    }
  };

  const saveSettings = async (newConfig: Config) => {
    try {
      await invoke("save_config", { config: newConfig });
      setConfig(newConfig);
      setShowSettings(false);
      // Regenerate QR if pin changed and server is running
      if (serverRunning) {
        generateQr(localIp, newConfig.port, newConfig.pin);
      }
    } catch (error) {
      console.error("Save Error:", error);
    }
  };

  return (
    <div className="app-container">
      <Header />

      <main className="dashboard">
        <Sidebar
          serverRunning={serverRunning}
          clientCount={clientCount}
          ip={localIp}
          port={config?.port || 8080}
          pin={config?.pin || "1234"}
          qrCode={qrCode}
          onStart={startServer}
          onStop={stopServer}
          onSettings={() => setShowSettings(true)}
        />

        {/* Console is always visible if there's space, or we can toggle it. 
            User wanted consistent size, suggesting the layout should be static.
            I'll keep the column there but maybe show a placeholder if not running? 
            Or just show Console always? 
            I'll show it always to fill the space. */}
        <div className="main-col">
          <Console />
        </div>
      </main>

      <SettingsModal
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
        config={config}
        onSave={saveSettings}
      />
    </div>
  );
}

export default App;
