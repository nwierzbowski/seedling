import { Handle, Position, type NodeProps } from '@xyflow/react';
import React from 'react';

export interface AgentNodeData {
  label: string;
  [key: string]: any;
}

// Base Agent class that defines common properties and methods for all agents
abstract class Agent extends React.Component<NodeProps<any>> {

  protected agentType: string = '';

  // Common styles for all agents
  protected getCommonStyles(): React.CSSProperties {
    return {
      padding: '10px',
      borderRadius: '4px',
      minWidth: '120px',
      textAlign: 'center',
      position: 'relative', // Necessary for handle positioning
    };
  }

  abstract getStyles(): React.CSSProperties;


  render() {
    const commonStyles = this.getCommonStyles();
    const agentStyles = this.getStyles();

    return (
      <div style={{ ...commonStyles, ...agentStyles }}>
        {/* Input Connection Point */}
        <Handle type="target" position={Position.Top} isConnectable={this.props.isConnectable} />
        
        {/* Specific Agent Content */}
        {this.renderContent()}

        {/* Output Connection Point */}
        <Handle type="source" position={Position.Bottom} isConnectable={this.props.isConnectable} />
      </div>
    );
  }

  // Subclasses will implement this instead of render()
  abstract renderContent(): React.ReactNode;
}

export default Agent;