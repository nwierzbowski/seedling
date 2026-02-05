import './AgentsTab.css';
import React, { useState } from 'react';
import ReactFlow, { Controls, Background, MiniMap } from 'reactflow';
import 'reactflow/dist/style.css';

// Define types for our nodes and edges
type NodeData = {
  label: string;
};

type Node = {
  id: string;
  type?: string;
  position: { x: number; y: number };
  data: NodeData;
};

type Edge = {
  id: string;
  source: string;
  target: string;
};

function AgentsTab() {
  // Define initial nodes and edges for our flow
  const [nodes] = useState<Node[]>([
    { id: '1', position: { x: 0, y: 0 }, data: { label: 'Engineer' } },
    { id: '2', position: { x: 250, y: 100 }, data: { label: 'Tester' } },
    { id: '3', position: { x: 500, y: 0 }, data: { label: 'Auditor' } },
  ]);

  const [edges] = useState<Edge[]>([
    { id: 'e1-2', source: '1', target: '2' },
    { id: 'e2-3', source: '2', target: '3' },
  ]);

  return (
    <div className="tab-content">
      <h2>Agents</h2>
      <p>Agent collaboration flow diagram:</p>

      {/* React Flow Diagram */}
      <div style={{ width: '100%', height: '400px', border: '1px solid #ccc', borderRadius: '4px' }}>
        <ReactFlow nodes={nodes} edges={edges}>
          <Background />
          <Controls />
          <MiniMap />
        </ReactFlow>
      </div>

      <div className="agent-list">
        <h3>Agent Roles:</h3>
        <ul>
          <li>Agent 1: Engineer</li>
          <li>Agent 2: Tester</li>
          <li>Agent 3: Auditor</li>
        </ul>
      </div>
    </div>
  );
}

export default AgentsTab;