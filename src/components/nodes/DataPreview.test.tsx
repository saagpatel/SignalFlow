import { afterEach, describe, expect, it } from "vitest";
import { DataPreview } from "./DataPreview";
import { useExecutionStore } from "../../stores/executionStore";
import { render } from "../../test/render";

const mountedViews: Array<{ unmount: () => void }> = [];

function mountPreview(nodeId: string) {
  const view = render(<DataPreview nodeId={nodeId} />);
  mountedViews.push(view);
  return view;
}

describe("DataPreview", () => {
  afterEach(() => {
    while (mountedViews.length > 0) {
      mountedViews.pop()?.unmount();
    }
    useExecutionStore.getState().reset();
  });

  it("truncates long string output", () => {
    const longText = "a".repeat(90);
    useExecutionStore.getState().setNodeOutput("node-1", longText);

    const view = mountPreview("node-1");

    expect(view.container.textContent).toBe(`${"a".repeat(80)}...`);
  });

  it("unwraps single-key objects into a readable preview", () => {
    useExecutionStore.getState().setNodeOutput("node-2", {
      result: ["alpha", "beta", "gamma"],
    });

    const view = mountPreview("node-2");

    expect(view.container.textContent).toBe("Array[3]: [alpha, beta, ...]");
  });

  it("shows truncated errors before output", () => {
    useExecutionStore.getState().setNodeOutput("node-3", "completed");
    const error = "boom ".repeat(20).trim();
    useExecutionStore.getState().setNodeError("node-3", error);

    const view = mountPreview("node-3");

    expect(view.container.textContent).toBe(error.slice(0, 60));
  });

  it("renders nothing when there is no output or error", () => {
    const view = mountPreview("node-4");

    expect(view.container.innerHTML).toBe("");
  });
});
