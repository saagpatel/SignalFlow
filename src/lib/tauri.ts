import { invoke, Channel } from "@tauri-apps/api/core";

export interface FlowDocument {
  id: string | null;
  name: string;
  nodes: FlowNode[];
  edges: FlowEdge[];
  viewport: { x: number; y: number; zoom: number };
}

export interface FlowNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: Record<string, unknown>;
}

export interface FlowEdge {
  id: string;
  source: string;
  target: string;
  sourceHandle: string | null;
  targetHandle: string | null;
}

export interface ExecutionEvent {
  type: "NodeStarted" | "NodeCompleted" | "NodeError" | "ExecutionComplete";
  node_id?: string;
  output_preview?: string;
  output_data?: unknown;
  duration_ms?: number;
  total_duration_ms?: number;
  error?: string;
}

export interface ExecutionResult {
  success: boolean;
  total_duration_ms: number;
  node_results: Record<
    string,
    {
      success: boolean;
      output_preview: string | null;
      error: string | null;
      duration_ms: number;
    }
  >;
  error: string | null;
}

export async function executeFlow(
  flow: FlowDocument,
  onProgress: (event: ExecutionEvent) => void,
): Promise<ExecutionResult> {
  const channel = new Channel<ExecutionEvent>();
  channel.onmessage = onProgress;
  return invoke("execute_flow", { flow, onProgress: channel });
}

export async function stopExecution(): Promise<void> {
  return invoke("stop_execution");
}

export async function saveFlow(flow: FlowDocument): Promise<string> {
  return invoke("save_flow", { flow });
}

export async function loadFlow(id: string): Promise<FlowDocument> {
  return invoke("load_flow", { id });
}

export interface FlowSummary {
  id: string;
  name: string;
  updated_at: string;
}

export async function listFlows(): Promise<FlowSummary[]> {
  return invoke("list_flows");
}

export async function deleteFlow(id: string): Promise<void> {
  return invoke("delete_flow", { id });
}

export interface OllamaStatus {
  available: boolean;
  error: string | null;
}

export interface ModelInfo {
  name: string;
  size: number;
  modified_at: string;
}

export async function checkOllama(endpoint?: string): Promise<OllamaStatus> {
  return invoke("check_ollama", { endpoint });
}

export async function listModels(endpoint?: string): Promise<ModelInfo[]> {
  return invoke("list_models", { endpoint });
}

export async function getPreference(key: string): Promise<string | null> {
  return invoke("get_preference", { key });
}

export async function setPreference(
  key: string,
  value: string | number | boolean,
): Promise<void> {
  return invoke("set_preference", { key, value: String(value) });
}

export async function getNodeDefinitions(): Promise<
  Array<{
    type: string;
    label: string;
    category: string;
    description: string;
  }>
> {
  return invoke("get_node_definitions");
}
