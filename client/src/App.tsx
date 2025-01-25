import { Button } from "antd";
import Canvas from "./pixi/Canvas";
import useExplorerDispatcher from "./api/explorer";
import { useSetAtom } from "jotai";
import dispatcherAtom from "./state/dispatcher";

const App = () => {
  const dispatcher = useExplorerDispatcher();
  useSetAtom(dispatcherAtom)(dispatcher);

  return (
    <div className="flex">
      <Canvas />
      <div>
        タイムライン
        <Button type="primary">Primary Button</Button>
      </div>
    </div>
  );
};

export default App;
