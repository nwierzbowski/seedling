import Agent from './AgentBase';
import React from 'react';

class TesterAgent extends Agent {
  constructor(props: any) {
    super(props);
    this.agentType = 'tester';
  }

  getStyles(): React.CSSProperties {
    return {
      border: '2px solid #4CAF50',
      backgroundColor: '#E8F5E9',
      color: '#2E7D32'
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

export default TesterAgent;