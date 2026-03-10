import {
  ReactFlow,
  type Connection,
  type Edge,
  type EdgeTypes,
  SelectionMode,
} from "@xyflow/react";
import { useCallback } from "react";
import { useFlowStore } from "../../stores/flowStore";
import { useUiStore } from "../../stores/uiStore";
import { useDragAndDrop } from "../../hooks/useDragAndDrop";
import { isValidConnection } from "../../lib/connectionValidator";
import { nodeTypes } from "../nodes/node-types";
import { AnimatedEdge } from "./AnimatedEdge";
import { FlowBackground } from "./FlowBackground";
import { CanvasControls } from "./CanvasControls";
import { CommandPalette } from "../command-palette/CommandPalette";
import { SNAP_GRID } from "../../lib/constants";

const edgeTypes: EdgeTypes = {
  animated: AnimatedEdge,
};

const proOptions = { hideAttribution: true };
const defaultEdgeOptions = { type: "animated" as const };
const deleteKeyCode = ["Backspace", "Delete"];

export function FlowCanvas() {
  const {
    nodes,
    edges,
    viewport,
    hasPersistedViewport,
    onNodesChange,
    onEdgesChange,
    onConnect,
    setViewport,
  } = useFlowStore();
  const selectNode = useUiStore((s) => s.selectNode);
  const { onDragOver, onDrop } = useDragAndDrop();

  const handleIsValidConnection = useCallback(
    (connection: Connection | Edge) => {
      return isValidConnection(connection, nodes, edges).valid;
    },
    [nodes, edges],
  );

  const showEmptyState = nodes.length === 0;

  return (
    <div className="h-full w-full">
      {showEmptyState && (
        <div className="pointer-events-none absolute inset-0 z-10 flex flex-col items-center justify-center gap-3">
          <p className="text-lg font-medium text-text-secondary">
            No nodes yet
          </p>
          <p className="text-sm text-text-secondary/70">
            Drag nodes from the palette or press{" "}
            <kbd className="rounded border border-panel-border bg-panel-bg px-1.5 py-0.5 text-xs">
              Cmd+K
            </kbd>{" "}
            to get started
          </p>
        </div>
      )}
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeClick={(_e, node) => selectNode(node.id)}
        onPaneClick={() => selectNode(null)}
        onDragOver={onDragOver}
        onDrop={onDrop}
        onMoveEnd={(event, nextViewport) => {
          if (event) {
            setViewport(nextViewport);
          }
        }}
        isValidConnection={handleIsValidConnection}
        selectionMode={SelectionMode.Partial}
        fitView={nodes.length > 0 && !hasPersistedViewport}
        defaultViewport={viewport}
        snapToGrid
        snapGrid={SNAP_GRID}
        proOptions={proOptions}
        defaultEdgeOptions={defaultEdgeOptions}
        minZoom={0.1}
        maxZoom={4}
        deleteKeyCode={deleteKeyCode}
      >
        <FlowBackground />
        <CanvasControls />
      </ReactFlow>
      <CommandPalette />
    </div>
  );
}
