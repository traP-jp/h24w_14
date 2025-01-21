import { Sprite, Stage } from "@pixi/react";
import World from "./World";
import { useEffect, useRef, useState } from "react";

interface CanvasProps {
  className?: string;
}

const Canvas = (props: CanvasProps) => {
  const [userPosition, setUserPosition] = useState({ x: 400, y: 300 });
  const [userTargetPosition, setUserTargetPosition] = useState({
    x: 400,
    y: 300,
  });
  const [fieldSize, setFieldSize] = useState({ width: 1000, height: 600 });
  const stageRef = useRef<HTMLDivElement>(null);

  const userDisplayPosition = {
    x: fieldSize.width / 2,
    y: fieldSize.height / 2,
  };

  useEffect(() => {
    const width = (window.innerWidth * 3) / 5;
    const height = window.innerHeight;

    setFieldSize({
      width: width,
      height: height,
    });
    setUserPosition({
      x: width / 2,
      y: height / 2,
    });
    setUserTargetPosition({ x: width / 2, y: height / 2 });
    // TODO: リサイズオブザーバー入れる
  }, []);

  const updatePosition = (
    position: { x: number; y: number },
    diff: { x: number; y: number }
  ): { x: number; y: number } => {
    const x = Math.max(Math.min(position.x + diff.x, 2000), 0);
    const y = Math.max(Math.min(position.y + diff.y, 2000), 0);
    return { x, y };
  };

  //TODO: setIntervalにする
  setTimeout(() => {
    setUserPosition((position) => {
      const diff = {
        x: userTargetPosition.x - position.x,
        y: userTargetPosition.y - position.y,
      };
      if (Math.abs(diff.x) < 1 && Math.abs(diff.y) < 1) {
        return userTargetPosition;
      }
      const nextPosition = updatePosition(position, {
        x: diff.x / 30,
        y: diff.y / 30,
      });
      return nextPosition;
    });
  }, 1000 / 60);

  return (
    <div ref={stageRef}>
      <Stage // Field
        {...fieldSize}
        options={{ background: 0x1099bb }}
        className={props.className}
        onClick={(e) => {
          if (stageRef.current === null) {
            return;
          }
          const stage = stageRef.current;
          const rect = stage.getBoundingClientRect();
          const clickDisplayPosition = {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top,
          };
          const clickWorldPosition = {
            x: clickDisplayPosition.x + userPosition.x - userDisplayPosition.x,
            y: clickDisplayPosition.y + userPosition.y - userDisplayPosition.y,
          };
          setUserTargetPosition(clickWorldPosition);
        }}
      >
        <World
          userPosition={userPosition}
          userDisplayPosition={{
            x: fieldSize.width / 2,
            y: fieldSize.height / 2,
          }}
        />
        <Sprite
          image={"https://pixijs.io/pixi-react/img/bunny.png"}
          {...userDisplayPosition}
        />
      </Stage>
    </div>
  );
};

export default Canvas;
