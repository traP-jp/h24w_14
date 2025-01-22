import Canvas from "./pixi/Canvas";
import { Timeline } from "./timeline/timeline";

const App = () => {
  return (
    <div className="flex">
      <Canvas />
      <Timeline />
    </div>
  );
};

export default App;
