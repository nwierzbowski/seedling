import React from 'react';
import { Handle, Position } from '@xyflow/react';

// Wrapper function to create Engineer Agent component
export const createEngineerAgent = () => {
  const EngineerAgentComponent = ({ data, id, isConnectable }: any) => {
    // Engineer-specific styles
    const agentStyles: React.CSSProperties = {
      border: '2px solid #2196F3',
      backgroundColor: '#E3F2FD',
      color: '#0D47A1',
      padding: '10px',
      borderRadius: '4px',
      minWidth: '120px',
      textAlign: 'center',
      position: 'relative'
    };

    // Render content specific to Engineer Agent
    const renderContent = () => {
      return (
        <div>
          <strong>{data?.label || 'Engineer Agent'}</strong>
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

  return EngineerAgentComponent;
};
