import { useCallback } from "react";
import { useFlowStore } from "../stores/flowStore";
import { useExecutionStore } from "../stores/executionStore";
import { useProjectStore } from "../stores/projectStore";
import {
  executeFlow,
  stopExecution,
  type ExecutionEvent,
  type FlowDocument,
} from "../lib/tauri";
import { validateFlow } from "../lib/flowValidator";
import { useToast } from "./useToast";

export function useExecution() {
  const nodes = useFlowStore((s) => s.nodes);
  const edges = useFlowStore((s) => s.edges);
  const viewport = useFlowStore((s) => s.viewport);
  const executionStatus = useExecutionStore((s) => s.status);
  const startExecution = useExecutionStore((s) => s.startExecution);
  const setNodeStatus = useExecutionStore((s) => s.setNodeStatus);
  const setNodeOutput = useExecutionStore((s) => s.setNodeOutput);
  const setNodeError = useExecutionStore((s) => s.setNodeError);
  const completeExecution = useExecutionStore((s) => s.completeExecution);
  const failExecution = useExecutionStore((s) => s.failExecution);
  const cancelExecution = useExecutionStore((s) => s.cancelExecution);
  const addLog = useExecutionStore((s) => s.addLog);
  const setValidationWarnings = useExecutionStore(
    (s) => s.setValidationWarnings,
  );
  const flowName = useProjectStore((s) => s.currentFlowName);
  const flowId = useProjectStore((s) => s.currentFlowId);
  const { toast } = useToast();

  const run = useCallback(async () => {
    if (executionStatus === "running") return;

    // Run pre-flight validation
    const warnings = validateFlow(nodes, edges);
    if (warnings.length > 0) {
      // Group warnings by node
      const grouped: Record<string, string[]> = {};
      for (const w of warnings) {
        if (!grouped[w.nodeId]) grouped[w.nodeId] = [];
        grouped[w.nodeId].push(w.message);
      }
      setValidationWarnings(grouped);

      const warningMessages = warnings.map((w) => w.message);
      toast({
        title: `${warnings.length} validation warning${warnings.length > 1 ? "s" : ""}`,
        description: warningMessages.slice(0, 3).join("; "),
        variant: "warning",
      });
    } else {
      setValidationWarnings({});
    }

    startExecution();
    addLog({ level: "info", message: "Execution started" });

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

    const handleEvent = (event: ExecutionEvent) => {
      switch (event.type) {
        case "NodeStarted":
          if (event.node_id) {
            setNodeStatus(event.node_id, "running");
            addLog({
              nodeId: event.node_id,
              level: "info",
              message: `Node ${event.node_id} started`,
            });
          }
          break;
        case "NodeCompleted":
          if (event.node_id) {
            setNodeStatus(event.node_id, "success");
            // Prefer full output data over preview string
            if (event.output_data != null) {
              setNodeOutput(event.node_id, event.output_data);
            } else if (event.output_preview) {
              setNodeOutput(event.node_id, event.output_preview);
            }
            addLog({
              nodeId: event.node_id,
              level: "info",
              message: `Node ${event.node_id} completed in ${event.duration_ms ?? 0}ms`,
            });
          }
          break;
        case "NodeError":
          if (event.node_id && event.error) {
            setNodeError(event.node_id, event.error);
            addLog({
              nodeId: event.node_id,
              level: "error",
              message: `Node ${event.node_id}: ${event.error}`,
            });
          }
          break;
        case "ExecutionComplete":
          break;
      }
    };

    try {
      const result = await executeFlow(flow, handleEvent);
      if (result.success) {
        completeExecution(result.total_duration_ms);
        addLog({
          level: "info",
          message: `Execution completed in ${result.total_duration_ms}ms`,
        });
        toast({
          title: "Execution complete",
          description: `Finished in ${result.total_duration_ms}ms`,
          variant: "success",
        });
      } else {
        failExecution(result.error ?? "Unknown error");
        toast({
          title: "Execution failed",
          description: result.error ?? "Unknown error",
          variant: "error",
        });
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      failExecution(msg);
      addLog({ level: "error", message: `Execution failed: ${msg}` });
      toast({ title: "Execution failed", description: msg, variant: "error" });
    }
  }, [
    addLog,
    completeExecution,
    edges,
    executionStatus,
    failExecution,
    flowId,
    flowName,
    nodes,
    setNodeError,
    setNodeOutput,
    setNodeStatus,
    setValidationWarnings,
    startExecution,
    toast,
    viewport,
  ]);

  const stop = useCallback(async () => {
    try {
      await stopExecution();
      cancelExecution();
      addLog({ level: "warn", message: "Execution cancelled" });
      toast({ title: "Execution cancelled", variant: "warning" });
    } catch (e) {
      // Still cancel locally even if backend call fails
      cancelExecution();
      const msg = e instanceof Error ? e.message : String(e);
      addLog({ level: "error", message: `Stop failed: ${msg}` });
      toast({ title: "Stop failed", description: msg, variant: "error" });
    }
  }, [addLog, cancelExecution, toast]);

  return { run, stop, status: executionStatus };
}
