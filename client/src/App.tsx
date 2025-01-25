import Canvas from "./pixi/Canvas";
import { Timeline } from "./timeline/Timeline";
import useExplorerDispatcher from "./api/explorer";
import { useSetAtom } from "jotai";
import dispatcherAtom from "./state/dispatcher";

const App = () => {
  const dispatcher = useExplorerDispatcher();
  useSetAtom(dispatcherAtom)(dispatcher);

  return (
    <div className="flex">
      <Canvas />
      <Timeline />
    </div>
  );
};

export default App;
