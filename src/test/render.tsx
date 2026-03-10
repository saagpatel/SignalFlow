import { act, type ReactNode } from "react";
import { createRoot } from "react-dom/client";

interface RenderResult {
  container: HTMLDivElement;
  rerender: (ui: ReactNode) => void;
  unmount: () => void;
}

export function render(ui: ReactNode): RenderResult {
  const container = document.createElement("div");
  document.body.appendChild(container);

  const root = createRoot(container);

  const renderWithRoot = (nextUi: ReactNode) => {
    act(() => {
      root.render(nextUi);
    });
  };

  renderWithRoot(ui);

  return {
    container,
    rerender: renderWithRoot,
    unmount: () => {
      act(() => {
        root.unmount();
      });
      container.remove();
    },
  };
}
