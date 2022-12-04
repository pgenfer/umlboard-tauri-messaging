import reactLogo from "./assets/react.svg";
import "./App.css";
import { useDispatch } from "react-redux";
import { useRootSelector } from "./store";
import { cancelNameChange, changeName, changingName } from "./edit-name.redux";

function App() {
  
  const name = useRootSelector(state => state.editName.currentName);
  const dispatch = useDispatch();

  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log('hello');
    // setGreetMsg(await invoke("greet", { name })); 
  }

  async function sendMessage<T>(action: {type: string, payload: T}) {
    const actionName = action.type.split("/")[0];
    const message = {
      [actionName]: action.payload
    };
    console.log(JSON.stringify(message));
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
            onChange={(e) => dispatch(changingName({newName: e.target.value}))}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={async () => {
            await sendMessage(changeName({newName: name}));
          }}>
            Edit
          </button>
          <button type="button" onClick={() => dispatch(cancelNameChange())}>
            Cancel
          </button>
        </div>
      </div>
      <p>{''}</p>
    </div>
  );
}

export default App;
