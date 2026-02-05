import React from 'react';
import { Handle, Position } from '@xyflow/react';

// Wrapper function to create Tester Agent component
export const createTesterAgent = () => {
  const TesterAgentComponent = ({ data, id, isConnectable }: any) => {
    // Tester-specific styles
    const agentStyles: React.CSSProperties = {
      border: '2px solid #4CAF50',
      backgroundColor: '#E8F5E9',
      color: '#2E7D32',
      padding: '10px',
      borderRadius: '4px',
      minWidth: '120px',
      textAlign: 'center',
      position: 'relative'
    };

    // Render content specific to Tester Agent
    const renderContent = () => {
      return (
        <div>
          <strong>{data?.label || 'Tester Agent'}</strong>
          <div style={{ fontSize: '10px' }}>ID: {id}</div>
        </div>
      );
    };

    return (
      <div style={agentStyles}>
        {/* Input Connection Point */}
        <Handle type="target" position={Position.Top} isConnectable={isConnectable} />

        {/* Specific Agent Content */}
        {renderContent()}

        {/* Output Connection Point */}
        <Handle type="source" position={Position.Bottom} isConnectable={isConnectable} />
      </div>
    );
  };

  return TesterAgentComponent;
};