import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useProjectStore } from "./projectStore";

describe("projectStore", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-03-10T10:00:00.000Z"));
    useProjectStore.setState({
      currentFlowId: null,
      currentFlowName: "Untitled Flow",
      isDirty: false,
      lastSavedAt: null,
      recentFlows: [],
      loading: true,
      activeScreen: "welcome",
    });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("marks flows as saved with a timestamp", () => {
    useProjectStore.getState().markSaved("flow-123");

    expect(useProjectStore.getState()).toMatchObject({
      currentFlowId: "flow-123",
      isDirty: false,
      lastSavedAt: "2026-03-10T10:00:00.000Z",
    });
  });

  it("switches between welcome and editor screens", () => {
    useProjectStore.getState().showEditor();
    expect(useProjectStore.getState().activeScreen).toBe("editor");

    useProjectStore.getState().showWelcome();
    expect(useProjectStore.getState().activeScreen).toBe("welcome");
  });
});
