import { ConfigProvider } from "antd";
import Canvas from "./pixi/Canvas";
import { Timeline } from "./timeline/Timeline";

const App = () => {
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
