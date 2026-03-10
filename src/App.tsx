import { useEffect, useCallback, useRef, useState } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import { FlowCanvas } from "./components/canvas/FlowCanvas";
import { NodePalette } from "./components/palette/NodePalette";
import { InspectorPanel } from "./components/panels/InspectorPanel";
import { ExecutionPanel } from "./components/panels/ExecutionPanel";
import { SettingsPanel } from "./components/panels/SettingsPanel";
import { TopToolbar } from "./components/toolbar/TopToolbar";
import { StatusBar } from "./components/toolbar/StatusBar";
import { useUiStore } from "./stores/uiStore";
import { useFlowStore } from "./stores/flowStore";
import { useProjectStore } from "./stores/projectStore";
import { useSettingsStore } from "./stores/settingsStore";
import { ErrorBoundary } from "./components/shared/ErrorBoundary";
import { ToastProvider } from "./components/shared/Toast";
import { WelcomeScreen } from "./components/welcome/WelcomeScreen";
import { useExecution } from "./hooks/useExecution";
import { useSaveFlow } from "./hooks/useSaveFlow";
import { useFlowManager } from "./hooks/useFlowManager";

function AppInner() {
  const isPaletteOpen = useUiStore((s) => s.isPaletteOpen);
  const isInspectorOpen = useUiStore((s) => s.isInspectorOpen);
  const isExecutionPanelOpen = useUiStore((s) => s.isExecutionPanelOpen);
  const loading = useProjectStore((s) => s.loading);
  const activeScreen = useProjectStore((s) => s.activeScreen);

  const { run } = useExecution();
  const { save } = useSaveFlow();
  const { loadLastFlow } = useFlowManager();
  const loadSettings = useSettingsStore((s) => s.loadSettings);
  const autoSaveInterval = useSettingsStore((s) => s.autoSaveInterval);

  // Settings modal state
  const [showSettings, setShowSettings] = useState(false);

  // Startup: load settings and last flow
  const startupDoneRef = useRef(false);
  useEffect(() => {
    if (!startupDoneRef.current) {
      startupDoneRef.current = true;
      void (async () => {
        await loadSettings();
        await loadLastFlow();
      })();
    }
  }, [loadLastFlow, loadSettings]);

  const nodes = useFlowStore((s) => s.nodes);
  const edges = useFlowStore((s) => s.edges);

  // Auto-save debounced
  const isDirty = useProjectStore((s) => s.isDirty);
  useEffect(() => {
    if (!isDirty || activeScreen !== "editor" || autoSaveInterval === 0) return;
    const timer = setTimeout(() => {
      void save();
    }, autoSaveInterval);
    return () => clearTimeout(timer);
  }, [activeScreen, autoSaveInterval, edges, isDirty, nodes, save]);

  // Clipboard for copy/paste
  const copySelected = useFlowStore((s) => s.copySelected);
  const pasteClipboard = useFlowStore((s) => s.pasteClipboard);
  const duplicateSelected = useFlowStore((s) => s.duplicateSelected);
  const selectAll = useFlowStore((s) => s.selectAll);

  // Keyboard shortcuts
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      const tag = (e.target as HTMLElement)?.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;

      const meta = e.metaKey || e.ctrlKey;

      if (!meta) return;

      switch (e.key) {
        case "s":
          e.preventDefault();
          save();
          break;
        case "Enter":
          e.preventDefault();
          run();
          break;
        case "z":
          e.preventDefault();
          if (e.shiftKey) {
            useFlowStore.temporal.getState().redo();
          } else {
            useFlowStore.temporal.getState().undo();
          }
          break;
        case "c":
          e.preventDefault();
          copySelected();
          break;
        case "v":
          e.preventDefault();
          pasteClipboard();
          break;
        case "d":
          e.preventDefault();
          duplicateSelected();
          break;
        case "a":
          e.preventDefault();
          selectAll();
          break;
      }
    },
    [save, run, copySelected, pasteClipboard, duplicateSelected, selectAll],
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [handleKeyDown]);

  // Show welcome screen when no flow is loaded and not loading
  const showWelcome = !loading && activeScreen === "welcome";

  return (
    <div className="flex h-screen flex-col bg-canvas-bg">
      <TopToolbar onSettingsClick={() => setShowSettings(true)} />

      <div className="relative flex flex-1 overflow-hidden">
        {isPaletteOpen && (
          <div className="flex w-56 flex-col border-r border-panel-border bg-panel-bg">
            <NodePalette />
          </div>
        )}

        <div className="flex flex-1 flex-col overflow-hidden">
          <div className="relative flex-1">
            <FlowCanvas />
            {loading && (
              <div className="absolute inset-0 z-20 flex items-center justify-center bg-canvas-bg/80 backdrop-blur-sm">
                <div className="rounded-lg border border-panel-border bg-panel-bg px-4 py-3 text-sm text-text-secondary shadow-lg">
                  Loading workspace...
                </div>
              </div>
            )}
            {showWelcome && <WelcomeScreen />}
          </div>

          {isExecutionPanelOpen && (
            <div className="h-48 border-t border-panel-border bg-panel-bg">
              <ExecutionPanel />
            </div>
          )}
        </div>

        {isInspectorOpen && (
          <div className="flex w-64 flex-col border-l border-panel-border bg-panel-bg">
            <InspectorPanel />
          </div>
        )}
      </div>

      <StatusBar />

      {showSettings && <SettingsPanel onClose={() => setShowSettings(false)} />}
    </div>
  );
}

export default function App() {
  return (
    <ErrorBoundary>
      <ToastProvider>
        <ReactFlowProvider>
          <AppInner />
        </ReactFlowProvider>
      </ToastProvider>
    </ErrorBoundary>
  );
}
