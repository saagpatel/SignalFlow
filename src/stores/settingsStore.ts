import { create } from "zustand";
import { getPreference, setPreference } from "../lib/tauri";

export const DEFAULT_OLLAMA_ENDPOINT = "http://localhost:11434";
export const DEFAULT_AUTO_SAVE_INTERVAL = 30000;
export type ThemeMode = "light" | "dark" | "auto";

interface SettingsStore {
  ollamaEndpoint: string;
  theme: ThemeMode;
  autoSaveInterval: number;

  setOllamaEndpoint: (endpoint: string) => Promise<void>;
  setTheme: (theme: ThemeMode) => Promise<void>;
  setAutoSaveInterval: (interval: number) => Promise<void>;
  loadSettings: () => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  // Default values
  ollamaEndpoint: DEFAULT_OLLAMA_ENDPOINT,
  theme: "auto",
  autoSaveInterval: DEFAULT_AUTO_SAVE_INTERVAL,

  setOllamaEndpoint: async (endpoint: string) => {
    try {
      await setPreference("ollama_endpoint", endpoint);
      set({ ollamaEndpoint: endpoint });
    } catch (error) {
      console.error("Failed to save Ollama endpoint:", error);
      throw error;
    }
  },

  setTheme: async (theme: ThemeMode) => {
    try {
      await setPreference("theme", theme);
      set({ theme });
      applyTheme(theme);
    } catch (error) {
      console.error("Failed to save theme:", error);
      throw error;
    }
  },

  setAutoSaveInterval: async (interval: number) => {
    try {
      await setPreference("auto_save_interval", interval);
      set({ autoSaveInterval: interval });
    } catch (error) {
      console.error("Failed to save auto-save interval:", error);
      throw error;
    }
  },

  loadSettings: async () => {
    try {
      const [endpoint, savedTheme, savedInterval] = await Promise.all([
        getPreference("ollama_endpoint").catch(() => null),
        getPreference("theme").catch(() => null),
        getPreference("auto_save_interval").catch(() => null),
      ]);

      const theme = isThemeMode(savedTheme) ? savedTheme : "auto";
      const interval = parseAutoSaveInterval(savedInterval);

      set({
        ollamaEndpoint: endpoint ?? DEFAULT_OLLAMA_ENDPOINT,
        theme,
        autoSaveInterval: interval,
      });

      applyTheme(theme);
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  },
}));

/**
 * Apply theme to document
 */
function applyTheme(theme: ThemeMode) {
  const root = document.documentElement;
  const resolvedTheme =
    theme === "auto"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : theme;

  root.classList.toggle("dark", resolvedTheme === "dark");
  root.classList.toggle("light", resolvedTheme === "light");
}

// Listen for system theme changes when in auto mode
if (typeof window !== "undefined") {
  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (e) => {
      const theme = useSettingsStore.getState().theme;
      if (theme === "auto") {
        const root = document.documentElement;
        root.classList.toggle("dark", e.matches);
        root.classList.toggle("light", !e.matches);
      }
    });
}

function isThemeMode(value: string | null): value is ThemeMode {
  return value === "light" || value === "dark" || value === "auto";
}

function parseAutoSaveInterval(value: string | null): number {
  if (value == null) return DEFAULT_AUTO_SAVE_INTERVAL;

  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed >= 0
    ? parsed
    : DEFAULT_AUTO_SAVE_INTERVAL;
}
