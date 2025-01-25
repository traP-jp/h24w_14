import { ConfigProvider } from "antd";
import Canvas from "./pixi/Canvas";
import { Timeline } from "./timeline/Timeline";
import { useEffect } from "react";
import useExplorerDispatcher from "./api/explorer";
import { useSetAtom } from "jotai";
import dispatcherAtom from "./state/dispatcher";

const App = () => {
  const dispatcher = useExplorerDispatcher();
  const setDispatcher = useSetAtom(dispatcherAtom);
  useEffect(() => {
    setDispatcher(dispatcher);
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
        <Timeline />
      </ConfigProvider>
    </div>
  );
};

export default App;
