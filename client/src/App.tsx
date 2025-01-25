import Canvas from "./pixi/Canvas";
import { Timeline } from "./timeline/Timeline";

const App = () => {
  return (
    <div className="flex">
      <Canvas />
      <Timeline />
    </div>
  );
};

export default App;
