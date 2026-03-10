import "@testing-library/jest-dom/vitest";

import { afterEach, vi } from "vitest";

(
  globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }
).IS_REACT_ACT_ENVIRONMENT = true;

const matchMediaListeners = new Set<(event: MediaQueryListEvent) => void>();
let prefersDarkMode = false;

Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: vi.fn().mockImplementation(() => ({
    matches: prefersDarkMode,
    media: "(prefers-color-scheme: dark)",
    onchange: null,
    addEventListener: (
      _event: string,
      listener: (event: MediaQueryListEvent) => void,
    ) => {
      matchMediaListeners.add(listener);
    },
    removeEventListener: (
      _event: string,
      listener: (event: MediaQueryListEvent) => void,
    ) => {
      matchMediaListeners.delete(listener);
    },
    addListener: (listener: (event: MediaQueryListEvent) => void) => {
      matchMediaListeners.add(listener);
    },
    removeListener: (listener: (event: MediaQueryListEvent) => void) => {
      matchMediaListeners.delete(listener);
    },
    dispatchEvent: () => true,
  })),
});

Object.assign(globalThis, {
  __setPreferredDarkMode(nextValue: boolean) {
    prefersDarkMode = nextValue;
    for (const listener of matchMediaListeners) {
      listener({ matches: nextValue } as MediaQueryListEvent);
    }
  },
});

afterEach(() => {
  prefersDarkMode = false;
  document.documentElement.classList.remove("dark", "light");
});
