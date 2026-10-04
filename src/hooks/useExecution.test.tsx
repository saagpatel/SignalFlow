import { act } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useExecution } from "./useExecution";
import { useExecutionStore } from "../stores/executionStore";
import type { ExecutionResult } from "../lib/tauri";
import { render } from "../test/render";

interface PendingRun {
  resolve: (result: ExecutionResult) => void;
  reject: (reason: unknown) => void;
}

const toast = vi.fn();
const pendingRuns: PendingRun[] = [];

vi.mock("./useToast", () => ({ useToast: () => ({ toast }) }));
vi.mock("../lib/tauri", () => ({
  executeFlow: vi.fn(
    () =>
      new Promise((resolve, reject) => {
        pendingRuns.push({ resolve, reject });
      }),
  ),
  stopExecution: vi.fn(() => Promise.resolve()),
}));

type ExecutionHook = ReturnType<typeof useExecution>;
let hook: ExecutionHook;
const capture = (value: ExecutionHook) => {
  hook = value;
};

function Harness({ onRender }: { onRender: (value: ExecutionHook) => void }) {
  onRender(useExecution());
  return null;
}

const okResult: ExecutionResult = {
  success: true,
  total_duration_ms: 5,
  node_results: {},
  error: null,
};

function toastTitles(): string[] {
  return toast.mock.calls.map((call) => (call[0] as { title: string }).title);
}

function startRun(): Promise<void> {
  let running!: Promise<void>;
  act(() => {
    running = hook.run();
  });
  return running;
}

describe("useExecution", () => {
  let view: { unmount: () => void };

  beforeEach(() => {
    useExecutionStore.getState().reset();
    toast.mockClear();
    pendingRuns.length = 0;
    view = render(<Harness onRender={capture} />);
  });

  afterEach(() => {
    view.unmount();
  });

  it("keeps a stopped run cancelled when the backend rejects it afterwards", async () => {
    const running = startRun();
    await act(async () => {
      await hook.stop();
    });
    await act(async () => {
      pendingRuns[0].reject("Execution cancelled");
      await running;
    });

    expect(useExecutionStore.getState().status).toBe("cancelled");
    expect(toastTitles()).toContain("Execution cancelled");
    expect(toastTitles()).not.toContain("Execution failed");
  });

  it("keeps a stopped run cancelled when the backend finishes it anyway", async () => {
    const running = startRun();
    await act(async () => {
      await hook.stop();
    });
    await act(async () => {
      pendingRuns[0].resolve(okResult);
      await running;
    });

    expect(useExecutionStore.getState().status).toBe("cancelled");
    expect(toastTitles()).not.toContain("Execution complete");
  });

  it("ignores a late rejection from a stopped run once a new run has started", async () => {
    const first = startRun();
    await act(async () => {
      await hook.stop();
    });
    const second = startRun();

    await act(async () => {
      pendingRuns[0].reject("Execution cancelled");
      await first;
    });
    expect(useExecutionStore.getState().status).toBe("running");

    await act(async () => {
      pendingRuns[1].resolve(okResult);
      await second;
    });
    expect(useExecutionStore.getState().status).toBe("complete");
  });

  it("still reports a real failure", async () => {
    const running = startRun();
    await act(async () => {
      pendingRuns[0].reject("Ollama unreachable");
      await running;
    });

    expect(useExecutionStore.getState().status).toBe("error");
    expect(toastTitles()).toContain("Execution failed");
  });

  it("clears the spinner of a node interrupted by Stop", () => {
    const store = useExecutionStore.getState();
    store.startExecution();
    store.setNodeStatus("done", "success");
    store.setNodeStatus("busy", "running");
    store.cancelExecution();

    const { nodeStatuses } = useExecutionStore.getState();
    expect(nodeStatuses).toEqual({ done: "success", busy: "idle" });
  });
});
