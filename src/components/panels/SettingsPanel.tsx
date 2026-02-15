import { useState, useEffect } from "react";
import { X, Check, AlertCircle } from "lucide-react";
import { useSettingsStore } from "../../stores/settingsStore";
import { invoke } from "@tauri-apps/api/core";

interface SettingsPanelProps {
  onClose: () => void;
}

export function SettingsPanel({ onClose }: SettingsPanelProps) {
  const {
    ollamaEndpoint,
    theme,
    autoSaveInterval,
    setOllamaEndpoint,
    setTheme,
    setAutoSaveInterval,
  } = useSettingsStore();

  const [endpoint, setEndpoint] = useState(ollamaEndpoint);
  const [ollamaStatus, setOllamaStatus] = useState<{
    checking: boolean;
    available: boolean;
    error?: string;
  }>({ checking: false, available: false });

  useEffect(() => {
    setEndpoint(ollamaEndpoint);
  }, [ollamaEndpoint]);

  const handleSaveEndpoint = async () => {
    try {
      await setOllamaEndpoint(endpoint);
      await checkOllamaHealth();
    } catch (error) {
      console.error("Failed to save endpoint:", error);
    }
  };

  const checkOllamaHealth = async () => {
    setOllamaStatus({ checking: true, available: false });
    try {
      const status = await invoke<{ available: boolean; error?: string }>(
        "check_ollama"
      );
      setOllamaStatus({ checking: false, available: status.available, error: status.error });
    } catch (error) {
      setOllamaStatus({
        checking: false,
        available: false,
        error: String(error),
      });
    }
  };

  const handleThemeChange = async (newTheme: "light" | "dark" | "auto") => {
    try {
      await setTheme(newTheme);
    } catch (error) {
      console.error("Failed to change theme:", error);
    }
  };

  const handleAutoSaveIntervalChange = async (interval: number) => {
    try {
      await setAutoSaveInterval(interval);
    } catch (error) {
      console.error("Failed to save auto-save interval:", error);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-md rounded-lg border border-panel-border bg-panel-bg p-6 shadow-xl">
        {/* Header */}
        <div className="mb-6 flex items-center justify-between">
          <h2 className="text-lg font-semibold text-text-primary">Settings</h2>
          <button
            onClick={onClose}
            className="rounded p-1 hover:bg-panel-border/50"
          >
            <X size={20} className="text-text-secondary" />
          </button>
        </div>

        {/* Settings Content */}
        <div className="space-y-6">
          {/* Ollama Endpoint */}
          <div>
            <label className="mb-2 block text-sm font-medium text-text-primary">
              Ollama Endpoint
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={endpoint}
                onChange={(e) => setEndpoint(e.target.value)}
                placeholder="http://localhost:11434"
                className="flex-1 rounded border border-panel-border bg-panel-bg px-3 py-2 text-sm text-text-primary outline-none focus:border-accent"
              />
              <button
                onClick={handleSaveEndpoint}
                className="rounded bg-accent px-4 py-2 text-sm font-medium text-white hover:bg-accent/90"
              >
                Save
              </button>
            </div>
            <button
              onClick={checkOllamaHealth}
              disabled={ollamaStatus.checking}
              className="mt-2 text-sm text-accent hover:underline disabled:opacity-50"
            >
              {ollamaStatus.checking ? "Checking..." : "Test Connection"}
            </button>
            {ollamaStatus.available && (
              <div className="mt-2 flex items-center gap-2 text-sm text-green-500">
                <Check size={16} />
                <span>Connected to Ollama</span>
              </div>
            )}
            {ollamaStatus.error && (
              <div className="mt-2 flex items-center gap-2 text-sm text-red-500">
                <AlertCircle size={16} />
                <span>{ollamaStatus.error}</span>
              </div>
            )}
          </div>

          {/* Theme */}
          <div>
            <label className="mb-2 block text-sm font-medium text-text-primary">
              Theme
            </label>
            <select
              value={theme}
              onChange={(e) =>
                handleThemeChange(e.target.value as "light" | "dark" | "auto")
              }
              className="w-full rounded border border-panel-border bg-panel-bg px-3 py-2 text-sm text-text-primary outline-none focus:border-accent"
            >
              <option value="auto">Auto (System)</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
            <p className="mt-1 text-xs text-text-secondary">
              Choose your preferred color scheme
            </p>
          </div>

          {/* Auto-Save Interval */}
          <div>
            <label className="mb-2 block text-sm font-medium text-text-primary">
              Auto-Save Interval
            </label>
            <select
              value={autoSaveInterval}
              onChange={(e) =>
                handleAutoSaveIntervalChange(Number(e.target.value))
              }
              className="w-full rounded border border-panel-border bg-panel-bg px-3 py-2 text-sm text-text-primary outline-none focus:border-accent"
            >
              <option value={10000}>10 seconds</option>
              <option value={30000}>30 seconds</option>
              <option value={60000}>1 minute</option>
              <option value={300000}>5 minutes</option>
              <option value={0}>Disabled</option>
            </select>
            <p className="mt-1 text-xs text-text-secondary">
              Automatically save your flow at regular intervals
            </p>
          </div>
        </div>

        {/* Footer */}
        <div className="mt-6 flex justify-end gap-2">
          <button
            onClick={onClose}
            className="rounded border border-panel-border px-4 py-2 text-sm font-medium text-text-primary hover:bg-panel-border/50"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
