import Agent from './AgentBase';
import React from 'react';

class EngineerAgent extends Agent {
  constructor(props: any) {
    super(props);
    // Configure the agent internally
    this.agentType = 'engineer';
  }

  getStyles(): React.CSSProperties {
    return {
      border: '2px solid #2196F3',
      backgroundColor: '#E3F2FD',
      color: '#0D47A1'
    };
  }

  renderContent() {
    return (
      <div>
        <strong>{this.props.data?.label || 'Tester Agent'}</strong>
        <div style={{ fontSize: '10px' }}>ID: {this.props.id}</div>
      </div>
    );
  }
}

export default EngineerAgent;