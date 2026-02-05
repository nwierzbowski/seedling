import { useState } from 'react';
import './App.css';
import TerminalTab from './components/TerminalTab';
import AgentsTab from './components/AgentsTab';
import SettingsTab from './components/SettingsTab';
import { ReactFlowProvider } from '@xyflow/react';

function App() {
  const [activeTab, setActiveTab] = useState('terminal');

  // Debugging to track tab switching
  const renderContent = () => {
    console.log('Rendering tab:', activeTab);
    switch (activeTab) {
      case 'terminal':
        return <TerminalTab key="terminal-tab" />;
      case 'agents':
        return <ReactFlowProvider ><AgentsTab key="agents-tab" /></ReactFlowProvider>;
      case 'settings':
        return <SettingsTab key="settings-tab" />;
      default:
        console.log('Default case triggered - this might be a problem');
        return <TerminalTab key="default-terminal" />;
    }
  };

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>🌱 AIDME (AI Development Management Environment)</h1>
        <div className="tabs-container">
          <button
            className={`tab ${activeTab === 'terminal' ? 'active' : ''}`}
            onClick={() => setActiveTab('terminal')}
          >
            Terminal
          </button>
          <button
            className={`tab ${activeTab === 'agents' ? 'active' : ''}`}
            onClick={() => setActiveTab('agents')}
          >
            Agents
          </button>
          <button
            className={`tab ${activeTab === 'settings' ? 'active' : ''}`}
            onClick={() => setActiveTab('settings')}
          >
            Settings
          </button>
        </div>
      </header>
      <main className="app-main">
        {renderContent()}
      </main>
    </div>
  )
}
export default App;