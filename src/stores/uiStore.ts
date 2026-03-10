import { create } from "zustand";

export interface UiState {
  selectedNodeId: string | null;
  isPaletteOpen: boolean;
  isInspectorOpen: boolean;
  isExecutionPanelOpen: boolean;
  selectNode: (id: string | null) => void;
  togglePalette: () => void;
  toggleInspector: () => void;
  toggleExecutionPanel: () => void;
}

export const useUiStore = create<UiState>()((set) => ({
  selectedNodeId: null,
  isPaletteOpen: true,
  isInspectorOpen: true,
  isExecutionPanelOpen: false,

  selectNode: (id) => set({ selectedNodeId: id }),
  togglePalette: () => set((s) => ({ isPaletteOpen: !s.isPaletteOpen })),
  toggleInspector: () => set((s) => ({ isInspectorOpen: !s.isInspectorOpen })),
  toggleExecutionPanel: () =>
    set((s) => ({ isExecutionPanelOpen: !s.isExecutionPanelOpen })),
}));
