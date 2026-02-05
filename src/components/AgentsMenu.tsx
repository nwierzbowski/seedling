import React, { useEffect, useRef } from "react";

interface AgentsMenuProps {
  position: { x: number; y: number };
  isVisible: boolean;
  onClose: () => void;
  onAddAgent: (agentType: string) => void;
}

const AgentsMenu: React.FC<AgentsMenuProps> = ({
  position,
  isVisible,
  onClose,
  onAddAgent
}) => {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    menuRef.current?.focus();
  })

  if (!isVisible) return null;

  return (
    <div
      ref={menuRef}
      onBlur={onClose}
      onMouseLeave={onClose}
      style={{
        position: 'absolute',
        left: position.x,
        top: position.y,
        backgroundColor: '#fff',
        border: '1px solid #ccc',
        borderRadius: '4px',
        boxShadow: '0 2px 8px rgba(0,0,0,0.15)',
        zIndex: 1000,
        padding: '10px',
        minWidth: '150px',
        transform: 'translate(-50%, -100%)', // Position above cursor
      }}
    >
      <div style={{ fontWeight: 'bold', marginBottom: '8px' }}>Agent Menu</div>
      <ul style={{ listStyle: 'none', margin: 0, padding: 0 }}>
        <li style={{ padding: '4px 8px', cursor: 'pointer' }} onClick={() => {
          onAddAgent("tester");
          onClose();
        }}>
          Add Tester Agent
        </li>
        <li style={{ padding: '4px 8px', cursor: 'pointer' }} onClick={() => {
          onAddAgent("engineer");
          onClose();
        }}>
          Add Engineer Agent
        </li>
        <li style={{ padding: '4px 8px', cursor: 'pointer' }} onClick={() => {
          onAddAgent("reviewer");
          onClose();
        }}>
          Add Reviewer Agent
        </li>
        <li style={{ padding: '4px 8px', cursor: 'pointer', borderTop: '1px solid #eee', marginTop: '4px' }} onClick={onClose}>
          Close Menu
        </li>
      </ul>
    </div>
  );
};

export default AgentsMenu;