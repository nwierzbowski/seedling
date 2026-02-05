import { useState, useCallback } from "react";
import {
  ReactFlow,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
  Background,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";

// Import the array of agent implementations
import TesterAgent from "./TesterAgent";
import EngineerAgent from "./EngineerAgent";
import ReviewerAgent from "./ReviewerAgent";

const initialNodes = [{id: 'node-1', data: {label: 'Tester Agent'}, type: 'tester', position: {x: 250, y: 5}}];

const initialEdges = [];

function AgentsTab() {
  const [nodes, setNodes] =
    useState(initialNodes);
  const [edges, setEdges] =
    useState(initialEdges);

  const onNodesChange = useCallback(
    (changes: any) =>
      setNodes(nodesSnapshot =>
        applyNodeChanges(changes, nodesSnapshot),
      ),
    [],
  );
  const onEdgesChange = useCallback(
    (changes: any) =>
      setEdges(edgesSnapshot =>
        applyEdgeChanges(changes, edgesSnapshot),
      ),
    [],
  );
  const onConnect = useCallback(
    (params: any) =>
      setEdges(edgesSnapshot =>
        addEdge(params, edgesSnapshot),
      ),
    [],
  );

  // Define node types using the agent implementations directly
  const nodeTypes = {
    tester: TesterAgent,
    engineer: EngineerAgent,
    reviewer: ReviewerAgent,
  };

  return (
    <div
      style={{ width: "100vw", height: "100vh" }}
    >
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
        nodeTypes={nodeTypes}
      >
        <Background />
      </ReactFlow>
    </div>
  );
}

export default AgentsTab;
