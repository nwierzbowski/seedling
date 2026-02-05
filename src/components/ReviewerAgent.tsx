import React from 'react';
import { Handle, Position } from '@xyflow/react';

// Wrapper function to create Reviewer Agent component
export const createReviewerAgent = () => {
  const ReviewerAgentComponent = ({ data, id, isConnectable }: any) => {
    // Reviewer-specific styles
    const agentStyles: React.CSSProperties = {
      border: '2px solid #FF9800',
      backgroundColor: '#FFF3E0',
      color: '#E65100',
      padding: '10px',
      borderRadius: '4px',
      minWidth: '120px',
      textAlign: 'center',
      position: 'relative'
    };

    // Render content specific to Reviewer Agent
    const renderContent = () => {
      return (
        <div>
          <strong>{data?.label || 'Reviewer Agent'}</strong>
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

  return ReviewerAgentComponent;
};
