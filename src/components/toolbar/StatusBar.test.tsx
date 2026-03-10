import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { Edge, Node } from "@xyflow/react";
import { StatusBar } from "./StatusBar";
import { useExecutionStore } from "../../stores/executionStore";
import { useFlowStore } from "../../stores/flowStore";
import { useProjectStore } from "../../stores/projectStore";
import { render } from "../../test/render";

const mountedViews: Array<{ unmount: () => void }> = [];

const defaultProjectState = {
  currentFlowId: null,
  currentFlowName: "Untitled Flow",
  isDirty: false,
  lastSavedAt: null,
  recentFlows: [],
  loading: false,
  activeScreen: "editor" as const,
};

const defaultFlowState = {
  nodes: [] as Node[],
  edges: [] as Edge[],
  viewport: { x: 0, y: 0, zoom: 1 },
  hasPersistedViewport: false,
  clipboard: null,
};

function mountStatusBar() {
  const view = render(<StatusBar />);
  mountedViews.push(view);
  return view;
}

describe("StatusBar", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-03-10T12:00:00.000Z"));
    useExecutionStore.getState().reset();
    useProjectStore.setState(defaultProjectState);
    useFlowStore.setState(defaultFlowState);
  });

  afterEach(() => {
    while (mountedViews.length > 0) {
      mountedViews.pop()?.unmount();
    }
    vi.useRealTimers();
  });

  it("shows flow counts, dirty state, relative save time, and ready status", () => {
    useFlowStore.setState({
      nodes: [
        { id: "n1", type: "textInput", position: { x: 0, y: 0 }, data: {} },
        { id: "n2", type: "debug", position: { x: 100, y: 0 }, data: {} },
      ],
      edges: [
        {
          id: "e1",
          source: "n1",
          target: "n2",
          sourceHandle: "value",
          targetHandle: "input",
        },
      ],
    });
    useProjectStore.setState({
      currentFlowName: "Demo Flow",
      isDirty: true,
      lastSavedAt: "2026-03-10T11:59:30.000Z",
    });

    const view = mountStatusBar();
    const text = view.container.textContent ?? "";

    expect(text).toContain("Demo Flow");
    expect(text).toContain("*");
    expect(text).toContain("2 nodes · 1 edge");
    expect(text).toContain("Saved 30s ago");
    expect(text).toContain("Ready");
  });

  it("shows completed execution timing", () => {
    useFlowStore.setState({
      nodes: [
        { id: "n1", type: "textInput", position: { x: 0, y: 0 }, data: {} },
      ],
    });
    useProjectStore.setState({ currentFlowName: "Finished Flow" });
    useExecutionStore.setState({ status: "complete", duration: 123 });

    const view = mountStatusBar();

    expect(view.container.textContent).toContain("1 node · 0 edges");
    expect(view.container.textContent).toContain("Completed in 123ms");
  });
});
