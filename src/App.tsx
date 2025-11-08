import { appWindow } from "@tauri-apps/api/window";

function App() {
  const handleClose = async () => {
    await appWindow.hide();
  };

  return (
    <div className="chat-container">
      <div className="chat-header" data-tauri-drag-region="true">
        <span className="chat-title">Oracle</span>
        <button
          className="chat-close"
          data-tauri-drag-region="false"
          onClick={handleClose}
        >
          Close
        </button>
      </div>
      <div className="chat-body">
        {/* Future chat UI goes here */}
      </div>
    </div>
  );
}

export default App;
