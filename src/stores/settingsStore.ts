import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

interface SettingsStore {
  ollamaEndpoint: string;
  theme: "light" | "dark" | "auto";
  autoSaveInterval: number;

  setOllamaEndpoint: (endpoint: string) => Promise<void>;
  setTheme: (theme: "light" | "dark" | "auto") => Promise<void>;
  setAutoSaveInterval: (interval: number) => Promise<void>;
  loadSettings: () => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  // Default values
  ollamaEndpoint: "http://localhost:11434",
  theme: "auto",
  autoSaveInterval: 30000, // 30 seconds

  setOllamaEndpoint: async (endpoint: string) => {
    try {
      await invoke("set_preference", {
        key: "ollama_endpoint",
        value: endpoint,
      });
      set({ ollamaEndpoint: endpoint });
    } catch (error) {
      console.error("Failed to save Ollama endpoint:", error);
      throw error;
    }
  },

  setTheme: async (theme: "light" | "dark" | "auto") => {
    try {
      await invoke("set_preference", {
        key: "theme",
        value: theme,
      });
      set({ theme });
      applyTheme(theme);
    } catch (error) {
      console.error("Failed to save theme:", error);
      throw error;
    }
  },

  setAutoSaveInterval: async (interval: number) => {
    try {
      await invoke("set_preference", {
        key: "auto_save_interval",
        value: interval,
      });
      set({ autoSaveInterval: interval });
    } catch (error) {
      console.error("Failed to save auto-save interval:", error);
      throw error;
    }
  },

  loadSettings: async () => {
    try {
      // Load Ollama endpoint
      const endpoint = await invoke<string>("get_preference", {
        key: "ollama_endpoint",
      }).catch(() => "http://localhost:11434");

      // Load theme
      const theme = (await invoke<string>("get_preference", {
        key: "theme",
      }).catch(() => "auto")) as "light" | "dark" | "auto";

      // Load auto-save interval
      const interval = await invoke<number>("get_preference", {
        key: "auto_save_interval",
      }).catch(() => 30000);

      set({
        ollamaEndpoint: endpoint,
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
function applyTheme(theme: "light" | "dark" | "auto") {
  const root = document.documentElement;

  if (theme === "dark") {
    root.classList.add("dark");
  } else if (theme === "light") {
    root.classList.remove("dark");
  } else {
    // Auto: use system preference
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    if (prefersDark) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }
}

// Listen for system theme changes when in auto mode
if (typeof window !== "undefined") {
  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (e) => {
      const theme = useSettingsStore.getState().theme;
      if (theme === "auto") {
        const root = document.documentElement;
        if (e.matches) {
          root.classList.add("dark");
        } else {
          root.classList.remove("dark");
        }
      }
    });
}
