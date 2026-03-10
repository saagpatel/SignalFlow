import { create } from "zustand";
import { temporal } from "zundo";
import {
  type Node,
  type Edge,
  type NodeChange,
  type EdgeChange,
  type OnNodesChange,
  type OnEdgesChange,
  type OnConnect,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
} from "@xyflow/react";
import { useProjectStore } from "./projectStore";

let pasteCounter = 0;

function generatePasteId(): string {
  return `paste_${Date.now()}_${++pasteCounter}`;
}

export interface FlowViewport {
  x: number;
  y: number;
  zoom: number;
}

const DEFAULT_VIEWPORT: FlowViewport = { x: 0, y: 0, zoom: 1 };

export interface FlowState {
  nodes: Node[];
  edges: Edge[];
  viewport: FlowViewport;
  hasPersistedViewport: boolean;
  clipboard: { nodes: Node[]; edges: Edge[] } | null;
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
  addNode: (node: Node) => void;
  removeNode: (id: string) => void;
  updateNodeConfig: (id: string, config: Record<string, unknown>) => void;
  setFlow: (
    nodes: Node[],
    edges: Edge[],
    viewport?: FlowViewport,
    options?: { persistedViewport?: boolean },
  ) => void;
  setViewport: (
    viewport: FlowViewport,
    options?: { markDirty?: boolean },
  ) => void;
  clear: () => void;
  copySelected: () => void;
  pasteClipboard: () => void;
  duplicateSelected: () => void;
  selectAll: () => void;
}

export const useFlowStore = create<FlowState>()(
  temporal(
    (set, get) => ({
      nodes: [],
      edges: [],
      viewport: DEFAULT_VIEWPORT,
      hasPersistedViewport: false,
      clipboard: null,

      onNodesChange: (changes) => {
        set({ nodes: applyNodeChanges(changes, get().nodes) });
        if (hasSubstantiveNodeChanges(changes)) {
          useProjectStore.getState().markDirty();
        }
      },

      onEdgesChange: (changes) => {
        set({ edges: applyEdgeChanges(changes, get().edges) });
        if (hasSubstantiveEdgeChanges(changes)) {
          useProjectStore.getState().markDirty();
        }
      },

      onConnect: (connection) => {
        set({ edges: addEdge(connection, get().edges) });
        useProjectStore.getState().markDirty();
      },

      addNode: (node) => {
        set({ nodes: [...get().nodes, node] });
        useProjectStore.getState().markDirty();
      },

      removeNode: (id) => {
        set({
          nodes: get().nodes.filter((n) => n.id !== id),
          edges: get().edges.filter((e) => e.source !== id && e.target !== id),
        });
        useProjectStore.getState().markDirty();
      },

      updateNodeConfig: (id, config) => {
        set({
          nodes: get().nodes.map((n) =>
            n.id === id ? { ...n, data: { ...n.data, ...config } } : n,
          ),
        });
        useProjectStore.getState().markDirty();
      },

      setFlow: (nodes, edges, viewport = DEFAULT_VIEWPORT, options) => {
        set({
          nodes,
          edges,
          viewport,
          hasPersistedViewport: options?.persistedViewport ?? false,
        });
      },

      setViewport: (viewport, options) => {
        set({ viewport });
        if (options?.markDirty !== false) {
          useProjectStore.getState().markDirty();
        }
      },

      clear: () => {
        set({
          nodes: [],
          edges: [],
          viewport: DEFAULT_VIEWPORT,
          hasPersistedViewport: false,
        });
      },

      copySelected: () => {
        const { nodes, edges } = get();
        const selectedNodes = nodes.filter((n) => n.selected);
        const selectedIds = new Set(selectedNodes.map((n) => n.id));
        const selectedEdges = edges.filter(
          (e) => selectedIds.has(e.source) && selectedIds.has(e.target),
        );
        if (selectedNodes.length > 0) {
          set({ clipboard: { nodes: selectedNodes, edges: selectedEdges } });
        }
      },

      pasteClipboard: () => {
        const { clipboard, nodes, edges } = get();
        if (!clipboard || clipboard.nodes.length === 0) return;

        const idMap = new Map<string, string>();
        const offset = 40;

        const newNodes = clipboard.nodes.map((n) => {
          const newId = generatePasteId();
          idMap.set(n.id, newId);
          return {
            ...n,
            id: newId,
            position: { x: n.position.x + offset, y: n.position.y + offset },
            selected: true,
          };
        });

        const newEdges = clipboard.edges
          .filter((e) => idMap.has(e.source) && idMap.has(e.target))
          .map((e) => ({
            ...e,
            id: generatePasteId(),
            source: idMap.get(e.source)!,
            target: idMap.get(e.target)!,
          }));

        // Deselect existing nodes
        const deselected = nodes.map((n) => ({ ...n, selected: false }));

        set({
          nodes: [...deselected, ...newNodes],
          edges: [...edges, ...newEdges],
        });
        useProjectStore.getState().markDirty();
      },

      duplicateSelected: () => {
        get().copySelected();
        get().pasteClipboard();
      },

      selectAll: () => {
        set({
          nodes: get().nodes.map((n) => ({ ...n, selected: true })),
          edges: get().edges.map((e) => ({ ...e, selected: true })),
        });
      },
    }),
    {
      limit: 50,
      partialize: (state) => ({
        nodes: state.nodes,
        edges: state.edges,
      }),
    },
  ),
);

function hasSubstantiveNodeChanges(changes: NodeChange[]): boolean {
  return changes.some((change) => change.type !== "select");
}

function hasSubstantiveEdgeChanges(changes: EdgeChange[]): boolean {
  return changes.some((change) => change.type !== "select");
}
