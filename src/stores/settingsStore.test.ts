import { beforeEach, describe, expect, it, vi } from "vitest";

const { getPreferenceMock, setPreferenceMock } = vi.hoisted(() => ({
  getPreferenceMock: vi.fn(),
  setPreferenceMock: vi.fn(),
}));

vi.mock("../lib/tauri", () => ({
  getPreference: getPreferenceMock,
  setPreference: setPreferenceMock,
}));

import {
  DEFAULT_AUTO_SAVE_INTERVAL,
  DEFAULT_OLLAMA_ENDPOINT,
  useSettingsStore,
} from "./settingsStore";

declare global {
  var __setPreferredDarkMode: (nextValue: boolean) => void;
}

describe("settingsStore", () => {
  beforeEach(() => {
    getPreferenceMock.mockReset();
    setPreferenceMock.mockReset();
    globalThis.__setPreferredDarkMode(false);
    useSettingsStore.setState({
      ollamaEndpoint: DEFAULT_OLLAMA_ENDPOINT,
      theme: "auto",
      autoSaveInterval: DEFAULT_AUTO_SAVE_INTERVAL,
    });
  });

  it("loads persisted settings and applies the saved theme", async () => {
    getPreferenceMock.mockImplementation(async (key: string) => {
      switch (key) {
        case "ollama_endpoint":
          return "http://localhost:22434";
        case "theme":
          return "dark";
        case "auto_save_interval":
          return "45000";
        default:
          return null;
      }
    });

    await useSettingsStore.getState().loadSettings();

    expect(useSettingsStore.getState()).toMatchObject({
      ollamaEndpoint: "http://localhost:22434",
      theme: "dark",
      autoSaveInterval: 45000,
    });
    expect(document.documentElement.classList.contains("dark")).toBe(true);
    expect(document.documentElement.classList.contains("light")).toBe(false);
  });

  it("falls back to defaults when persisted values are invalid", async () => {
    getPreferenceMock.mockImplementation(async (key: string) => {
      switch (key) {
        case "ollama_endpoint":
          return null;
        case "theme":
          return "sepia";
        case "auto_save_interval":
          return "not-a-number";
        default:
          return null;
      }
    });

    await useSettingsStore.getState().loadSettings();

    expect(useSettingsStore.getState()).toMatchObject({
      ollamaEndpoint: DEFAULT_OLLAMA_ENDPOINT,
      theme: "auto",
      autoSaveInterval: DEFAULT_AUTO_SAVE_INTERVAL,
    });
    expect(document.documentElement.classList.contains("light")).toBe(true);
  });

  it("persists auto-save interval updates", async () => {
    await useSettingsStore.getState().setAutoSaveInterval(0);

    expect(setPreferenceMock).toHaveBeenCalledWith("auto_save_interval", 0);
    expect(useSettingsStore.getState().autoSaveInterval).toBe(0);
  });

  it("persists Ollama endpoint updates", async () => {
    await useSettingsStore
      .getState()
      .setOllamaEndpoint("http://localhost:33434");

    expect(setPreferenceMock).toHaveBeenCalledWith(
      "ollama_endpoint",
      "http://localhost:33434",
    );
    expect(useSettingsStore.getState().ollamaEndpoint).toBe(
      "http://localhost:33434",
    );
  });

  it("persists theme updates and applies the selected mode", async () => {
    await useSettingsStore.getState().setTheme("light");

    expect(setPreferenceMock).toHaveBeenCalledWith("theme", "light");
    expect(document.documentElement.classList.contains("light")).toBe(true);
    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });

  it("falls back when stored preference lookups reject", async () => {
    getPreferenceMock.mockRejectedValue(new Error("ipc unavailable"));

    await useSettingsStore.getState().loadSettings();

    expect(useSettingsStore.getState()).toMatchObject({
      ollamaEndpoint: DEFAULT_OLLAMA_ENDPOINT,
      theme: "auto",
      autoSaveInterval: DEFAULT_AUTO_SAVE_INTERVAL,
    });
  });

  it("uses dark mode when auto theme loads while the system prefers dark", async () => {
    globalThis.__setPreferredDarkMode(true);
    getPreferenceMock.mockImplementation(async (key: string) => {
      switch (key) {
        case "theme":
          return "auto";
        default:
          return null;
      }
    });

    await useSettingsStore.getState().loadSettings();

    expect(document.documentElement.classList.contains("dark")).toBe(true);
  });

  it("reacts to system theme changes when using auto mode", async () => {
    getPreferenceMock.mockImplementation(async (key: string) => {
      switch (key) {
        case "theme":
          return "auto";
        default:
          return null;
      }
    });

    await useSettingsStore.getState().loadSettings();
    globalThis.__setPreferredDarkMode(true);

    expect(document.documentElement.classList.contains("dark")).toBe(true);
    expect(document.documentElement.classList.contains("light")).toBe(false);
  });

  it("rethrows errors when saving the theme fails", async () => {
    const error = new Error("save failed");
    setPreferenceMock.mockRejectedValue(error);

    await expect(useSettingsStore.getState().setTheme("dark")).rejects.toThrow(
      "save failed",
    );
  });
});
