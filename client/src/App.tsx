import { ConfigProvider } from "antd";
import { useSetAtom } from "jotai";
import { useEffect } from "react";
import useExplorerDispatcher from "./api/explorer";
import { ReactionPicker } from "./components/RactionPicker";
import { Timeline } from "./components/Timeline";
import Canvas from "./pixi/Canvas";
import dispatcherAtom from "./state/dispatcher";

const App = () => {
  const dispatcher = useExplorerDispatcher();
  const setDispatcher = useSetAtom(dispatcherAtom);
  useEffect(() => {
    setDispatcher(() => dispatcher);
  }, [dispatcher, setDispatcher]);

  return (
    <div className="flex">
      <Canvas />
      <ConfigProvider
        theme={{
          token: {
            colorPrimary: "#a0d911",
          },
        }}
      >
        <div
          className="
            absolute
            left-1/2
            -translate-x-1/2
            bottom-4
          "
        >
          <ReactionPicker />
        </div>
        <Timeline />
      </ConfigProvider>
    </div>
  );
};

export default App;
