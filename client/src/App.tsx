import { Button } from "antd";
import Canvas from "./pixi/Canvas";
import useExplorerDispatcher from "./api/explorer";
import { useSetAtom } from "jotai";
import dispatcherAtom from "./state/dispatcher";
import { useEffect } from "react";

const App = () => {
  const dispatcher = useExplorerDispatcher();
  const setDispatcher = useSetAtom(dispatcherAtom);
  useEffect(() => {
    setDispatcher(dispatcher);
  }, [dispatcher, setDispatcher]);

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
