import { useCallback, useRef } from "react";
import { useFlowStore } from "../stores/flowStore";
import { useProjectStore } from "../stores/projectStore";
import { saveFlow, setPreference, type FlowDocument } from "../lib/tauri";
import { useToast } from "./useToast";

export function useSaveFlow() {
  const nodes = useFlowStore((s) => s.nodes);
  const edges = useFlowStore((s) => s.edges);
  const viewport = useFlowStore((s) => s.viewport);
  const flowId = useProjectStore((s) => s.currentFlowId);
  const flowName = useProjectStore((s) => s.currentFlowName);
  const markSaved = useProjectStore((s) => s.markSaved);
  const isSaving = useRef(false);
  const { toast } = useToast();

  const save = useCallback(async () => {
    if (isSaving.current || nodes.length === 0) return;
    isSaving.current = true;

    const flow: FlowDocument = {
      id: flowId,
      name: flowName,
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
      const id = await saveFlow(flow);
      markSaved(id);
      await setPreference("lastOpenFlowId", id);
      toast({ title: "Flow saved", variant: "success" });
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      toast({ title: "Save failed", description: msg, variant: "error" });
    } finally {
      isSaving.current = false;
    }
  }, [edges, flowId, flowName, markSaved, nodes, toast, viewport]);

  return { save };
}
