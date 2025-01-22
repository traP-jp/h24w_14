import "./App.css";

import { Timeline } from "./timeline/timeline";
import Canvas from "./pixi/Canvas";

const App = () => {
  return (
    <div className="flex">
      <Canvas />
      <Timeline />
    </div>
  );
};

export default App;
