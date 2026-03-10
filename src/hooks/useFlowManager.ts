import { useCallback } from "react";
import { useFlowStore } from "../stores/flowStore";
import { useProjectStore } from "../stores/projectStore";
import { useExecutionStore } from "../stores/executionStore";
import {
  loadFlow,
  listFlows,
  deleteFlow,
  saveFlow,
  getPreference,
  setPreference,
  type FlowDocument,
} from "../lib/tauri";
import { useToast } from "./useToast";
import type { Node, Edge } from "@xyflow/react";

const LAST_FLOW_KEY = "lastOpenFlowId";

function flowDocToReactFlow(doc: FlowDocument): {
  nodes: Node[];
  edges: Edge[];
} {
  const nodes: Node[] = doc.nodes.map((n) => ({
    id: n.id,
    type: n.type,
    position: n.position,
    data: n.data,
  }));

  const edges: Edge[] = doc.edges.map((e) => ({
    id: e.id,
    source: e.source,
    target: e.target,
    sourceHandle: e.sourceHandle ?? undefined,
    targetHandle: e.targetHandle ?? undefined,
  }));

  return { nodes, edges };
}

export function useFlowManager() {
  const setFlow = useFlowStore((s) => s.setFlow);
  const clear = useFlowStore((s) => s.clear);
  const nodes = useFlowStore((s) => s.nodes);
  const edges = useFlowStore((s) => s.edges);
  const viewport = useFlowStore((s) => s.viewport);
  const setCurrentFlow = useProjectStore((s) => s.setCurrentFlow);
  const setRecentFlows = useProjectStore((s) => s.setRecentFlows);
  const setLoading = useProjectStore((s) => s.setLoading);
  const showEditor = useProjectStore((s) => s.showEditor);
  const showWelcome = useProjectStore((s) => s.showWelcome);
  const currentFlowId = useProjectStore((s) => s.currentFlowId);
  const markSaved = useProjectStore((s) => s.markSaved);
  const resetExecution = useExecutionStore((s) => s.reset);
  const { toast } = useToast();

  const refreshFlowList = useCallback(async () => {
    try {
      const flows = await listFlows();
      setRecentFlows(flows);
    } catch {
      // Silent failure — list is non-critical
    }
  }, [setRecentFlows]);

  const openFlow = useCallback(
    async (id: string) => {
      try {
        const doc = await loadFlow(id);
        const { nodes: rfNodes, edges: rfEdges } = flowDocToReactFlow(doc);
        setFlow(rfNodes, rfEdges, doc.viewport, { persistedViewport: true });
        setCurrentFlow(doc.id, doc.name);
        showEditor();
        resetExecution();
        await setPreference(LAST_FLOW_KEY, id);
        toast({ title: `Loaded "${doc.name}"`, variant: "success" });
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        toast({
          title: "Failed to load flow",
          description: msg,
          variant: "error",
        });
      }
    },
    [resetExecution, setCurrentFlow, setFlow, showEditor, toast],
  );

  const newFlow = useCallback(() => {
    clear();
    setCurrentFlow(null, "Untitled Flow");
    showEditor();
    resetExecution();
    toast({ title: "New flow created", variant: "info" });
  }, [clear, resetExecution, setCurrentFlow, showEditor, toast]);

  const removeFlow = useCallback(
    async (id: string) => {
      try {
        await deleteFlow(id);
        if (currentFlowId === id) {
          clear();
          setCurrentFlow(null, "Untitled Flow");
          showWelcome();
          resetExecution();
        }
        await refreshFlowList();
        toast({ title: "Flow deleted", variant: "success" });
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        toast({ title: "Delete failed", description: msg, variant: "error" });
      }
    },
    [
      currentFlowId,
      clear,
      refreshFlowList,
      resetExecution,
      setCurrentFlow,
      showWelcome,
      toast,
    ],
  );

  const saveAs = useCallback(
    async (name: string) => {
      if (nodes.length === 0) return;

      const flow: FlowDocument = {
        id: null, // Force new ID
        name,
        nodes: nodes.map((n) => ({
          id: n.id,
          type: n.type ?? "unknown",
          position: n.position,
          data: (n.data ?? {}) as Record<string, unknown>,
        })),
        edges: edges.map((e) => ({
          id: e.id,
          source: e.source,
          target: e.target,
          sourceHandle: e.sourceHandle ?? null,
          targetHandle: e.targetHandle ?? null,
        })),
        viewport,
      };

      try {
        const newId = await saveFlow(flow);
        setCurrentFlow(newId, name);
        showEditor();
        markSaved(newId);
        await setPreference(LAST_FLOW_KEY, newId);
        await refreshFlowList();
        toast({ title: `Saved as "${name}"`, variant: "success" });
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        toast({ title: "Save failed", description: msg, variant: "error" });
      }
    },
    [
      edges,
      markSaved,
      nodes,
      refreshFlowList,
      setCurrentFlow,
      showEditor,
      toast,
      viewport,
    ],
  );

  const loadLastFlow = useCallback(async () => {
    setLoading(true);
    try {
      await refreshFlowList();
      const lastId = await getPreference(LAST_FLOW_KEY);
      if (lastId) {
        await openFlow(lastId);
      } else {
        showWelcome();
      }
    } catch {
      // Silent — fall through to welcome screen
      showWelcome();
    } finally {
      setLoading(false);
    }
  }, [openFlow, refreshFlowList, setLoading, showWelcome]);

  return {
    openFlow,
    newFlow,
    removeFlow,
    saveAs,
    refreshFlowList,
    loadLastFlow,
  };
}
