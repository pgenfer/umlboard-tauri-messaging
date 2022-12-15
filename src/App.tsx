import reactLogo from "./assets/react.svg";
import "./App.css";
import { useDispatch } from "react-redux";
import { useRootSelector } from "./store";
import { invoke } from "@tauri-apps/api";
import { cancelClassifierRename, renameClassifier, renamingClassifier } from "./edit-name.redux";

type IpcMessage = {
  domain: string,
  action: {type: string, payload: any}
}

function App() {
  
  const name = useRootSelector(state => state.editName.currentName);
  const dispatch = useDispatch();

  async function sendMessage<T>(action: {type: string, payload: T}) {
    const domain = action.type.split("/")[0]
    const type = action.type.split("/")[1];
    const message: IpcMessage = {
      domain,
      action: {...action, type }
    };
    const answer = await invoke<IpcMessage>("ipc_message", {message} );
    const responseAction = {
      ...answer.action,
      type: `${answer.domain}/${answer.action.type}`
    };
    dispatch(responseAction);
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <div>
          <input
            id="greet-input"
            value={name}
            onChange={(e) => dispatch(renamingClassifier({newName: e.target.value}))}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={async () => {
            await sendMessage(renameClassifier({newName: name}));
          }}>
            Edit
          </button>
          <button type="button" onClick={async () => {
            await sendMessage(cancelClassifierRename());
          }}>
            Cancel
          </button>
        </div>
      </div>
      <p>{''}</p>
    </div>
  );
}

export default App;
