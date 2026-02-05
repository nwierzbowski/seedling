import './SettingsTab.css';

function SettingsTab() {
  return (
    <div className="tab-content">
      <h2>Settings</h2>
      <p>This is the settings tab content.</p>
      <form>
        <div>
          <label>
            Theme:
            <select>
              <option>Dark</option>
              <option>Light</option>
            </select>
          </label>
        </div>
        <div>
          <label>
            Auto-save:
            <input type="checkbox" />
          </label>
        </div>
      </form>
    </div>
  );
}

export default SettingsTab;