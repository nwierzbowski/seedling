import {
  useState,
  useCallback,
  useEffect,
  useRef,
} from "react";
import {
  ReactFlow,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
  Background,
  useReactFlow,
  ReactFlowProvider,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";

// Import the array of agent implementations
import { createTesterAgent } from "./TesterAgent";
import { createEngineerAgent } from "./EngineerAgent";
import { createReviewerAgent } from "./ReviewerAgent";
import AgentsMenu from "./AgentsMenu";
import { useShiftAHotkey } from "../hooks/useShiftAHotkey";

const initialNodes = [
  {
    id: "node-1",
    data: { label: "Tester Agent" },
    type: "tester",
    position: { x: 250, y: 5 },
  },
  {
    id: "node-2",
    data: { label: "Engineer Agent" },
    type: "engineer",
    position: { x: 100, y: 200 },
  },
  {
    id: "node-3",
    data: { label: "Reviewer Agent" },
    type: "reviewer",
    position: { x: 400, y: 200 },
  },
];

function AgentsTab() {
  const [nodes, setNodes] =
    useState<any[]>(initialNodes);
  const [edges, setEdges] = useState<any[]>([]);
  const [menuPosition, setMenuPosition] =
    useState({ x: 0, y: 0 });
    
  const { screenToFlowPosition } = useReactFlow();

  const mousePos = useRef({ x: 0, y: 0 });
  const onPaneMouseMove = useCallback(
    (event: React.MouseEvent) => {
      mousePos.current = {
        x: event.clientX,
        y: event.clientY,
      };
    },
    [],
  );

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
    tester: createTesterAgent(),
    engineer: createEngineerAgent(),
    reviewer: createReviewerAgent(),
  };

  // Handle keyboard shortcuts with custom hook
  const [showMenu, setShowMenu] =
    useShiftAHotkey();

  // Set menu position only when it opens (not following mouse)
  useEffect(() => {
    if (showMenu) {
      setMenuPosition(
        {
          x: mousePos.current.x,
          y: mousePos.current.y,
        },
      );
    }
  }, [showMenu]);

  // Handle adding agents
  const handleAddAgent = (agentType: string) => {
    const newNode = {
      id: `node-${Date.now()}`,
      data: {
        label: `${agentType.charAt(0).toUpperCase() + agentType.slice(1)} Agent`,
      },
      type: agentType,
      position: screenToFlowPosition({
        x: menuPosition.x,
        y: menuPosition.y + 20,
      }),
    };
    setNodes(prev => [...prev, newNode]);
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
        onPaneMouseMove={onPaneMouseMove}
        onConnect={onConnect}
        fitView
        nodeTypes={nodeTypes}
      >
        <AgentsMenu
          position={menuPosition}
          isVisible={showMenu}
          onClose={() => setShowMenu(false)}
          onAddAgent={handleAddAgent}
        />
        <Background />
      </ReactFlow>
    </div>
  );
}

export default AgentsTab;
