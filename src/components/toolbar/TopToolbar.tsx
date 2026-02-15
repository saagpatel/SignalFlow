import {
  Play,
  Square,
  Save,
  PanelLeft,
  PanelRight,
  Terminal,
  Undo2,
  Redo2,
  Sun,
  Moon,
  Settings,
} from "lucide-react";
import { useUiStore } from "../../stores/uiStore";
import { useFlowStore } from "../../stores/flowStore";
import { useProjectStore } from "../../stores/projectStore";
import { useExecution } from "../../hooks/useExecution";
import { useSaveFlow } from "../../hooks/useSaveFlow";

interface TopToolbarProps {
  onSettingsClick?: () => void;
}

export function TopToolbar({ onSettingsClick }: TopToolbarProps) {
  const togglePalette = useUiStore((s) => s.togglePalette);
  const toggleInspector = useUiStore((s) => s.toggleInspector);
  const toggleExecutionPanel = useUiStore((s) => s.toggleExecutionPanel);
  const toggleTheme = useUiStore((s) => s.toggleTheme);
  const theme = useUiStore((s) => s.theme);
  const flowName = useProjectStore((s) => s.currentFlowName);
  const isDirty = useProjectStore((s) => s.isDirty);
  const setFlowName = useProjectStore((s) => s.setFlowName);
  const nodes = useFlowStore((s) => s.nodes);

  const undo = useFlowStore.temporal.getState().undo;
  const redo = useFlowStore.temporal.getState().redo;

  const { run, stop, status } = useExecution();
  const { save } = useSaveFlow();

  return (
    <div className="flex h-10 items-center justify-between border-b border-panel-border bg-panel-bg px-3">
      <div className="flex items-center gap-2">
        <button
          onClick={togglePalette}
          className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
          aria-label="Toggle Node Palette"
          title="Toggle Node Palette"
        >
          <PanelLeft size={16} aria-hidden="true" />
        </button>

        <input
          className="w-40 border-b border-transparent bg-transparent text-sm font-medium text-text-primary focus:border-accent focus:outline-none"
          value={flowName}
          onChange={(e) => setFlowName(e.target.value)}
          aria-label="Flow name"
        />
        {isDirty && (
          <span className="text-xs text-text-secondary" title="Unsaved changes" aria-label="Unsaved changes">
            *
          </span>
        )}

        <div className="ml-2 flex items-center gap-1 border-l border-panel-border pl-2">
          <button
            onClick={() => undo()}
            className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
            aria-label="Undo"
            title="Undo (Cmd+Z)"
          >
            <Undo2 size={14} aria-hidden="true" />
          </button>
          <button
            onClick={() => redo()}
            className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
            aria-label="Redo"
            title="Redo (Cmd+Shift+Z)"
          >
            <Redo2 size={14} aria-hidden="true" />
          </button>
        </div>
      </div>

      <div className="flex items-center gap-2">
        {status === "running" ? (
          <button
            onClick={stop}
            className="flex items-center gap-1.5 rounded bg-red-500/20 px-3 py-1.5 text-xs font-medium text-red-400 hover:bg-red-500/30"
            aria-label="Stop execution"
          >
            <Square size={12} aria-hidden="true" />
            Stop
          </button>
        ) : (
          <button
            onClick={run}
            disabled={nodes.length === 0}
            className="flex items-center gap-1.5 rounded bg-green-500/20 px-3 py-1.5 text-xs font-medium text-green-400 hover:bg-green-500/30 disabled:opacity-40 disabled:cursor-not-allowed"
            aria-label="Run flow"
          >
            <Play size={12} aria-hidden="true" />
            Run
          </button>
        )}

        <div className="flex items-center gap-1 border-l border-panel-border pl-2">
          <button
            onClick={save}
            className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
            aria-label="Save flow"
            title="Save (Cmd+S)"
          >
            <Save size={14} aria-hidden="true" />
          </button>
          <button
            onClick={toggleExecutionPanel}
            className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
            aria-label="Toggle Execution Panel"
            title="Toggle Execution Panel"
          >
            <Terminal size={14} aria-hidden="true" />
          </button>
          <button
            onClick={toggleInspector}
            className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
            aria-label="Toggle Inspector"
            title="Toggle Inspector"
          >
            <PanelRight size={14} aria-hidden="true" />
          </button>
          <button
            onClick={toggleTheme}
            className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
            aria-label="Toggle Theme"
            title="Toggle Theme"
          >
            {theme === "dark" ? <Sun size={14} aria-hidden="true" /> : <Moon size={14} aria-hidden="true" />}
          </button>
          {onSettingsClick && (
            <button
              onClick={onSettingsClick}
              className="rounded p-1.5 text-text-secondary hover:bg-panel-border hover:text-text-primary"
              aria-label="Settings"
              title="Settings"
            >
              <Settings size={14} aria-hidden="true" />
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
