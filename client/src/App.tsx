import "./App.css";
import { Button } from "antd";
import Canvas from "./pixi/Canvas";

const App = () => {
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
