import Agent from './AgentBase';
import React from 'react';

class ReviewerAgent extends Agent {
  constructor(props: any) {
    super(props);
    this.agentType = 'reviewer';
  }

  getStyles(): React.CSSProperties {
    return {
      border: '2px solid #FF9800',
      backgroundColor: '#FFF3E0',
      color: '#E65100'
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

export default ReviewerAgent;