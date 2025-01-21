import { Sprite, Stage } from "@pixi/react";
import World from "./World";
import { useEffect, useRef, useState } from "react";

interface CanvasProps {
  className?: string;
}

const Canvas = (props: CanvasProps) => {
  const [userPosition, setUserPosition] = useState({ x: 400, y: 300 });
  const [fieldSize, setFieldSize] = useState({ width: 1000, height: 600 });
  const [intervalID, setIntervalID] = useState<number | null>(null);
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
    // TODO: リサイズオブザーバー入れる
  }, []);

  const calcNewPosition = (
    position: { x: number; y: number },
    diff: { x: number; y: number }
  ): { x: number; y: number } => {
    const x = Math.max(Math.min(position.x + diff.x, 2000), 0);
    const y = Math.max(Math.min(position.y + diff.y, 2000), 0);
    return { x, y };
  };

  const onFieldClick = (e: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
    if (stageRef.current === null) {
      return;
    }
    const stage = stageRef.current;
    const stageRect = stage.getBoundingClientRect();
    const clickDisplayPosition = {
      x: e.clientX - stageRect.left,
      y: e.clientY - stageRect.top,
    };

    const clickWorldPosition = {
      x: clickDisplayPosition.x + userPosition.x - userDisplayPosition.x,
      y: clickDisplayPosition.y + userPosition.y - userDisplayPosition.y,
    };

    if (intervalID !== null) {
      clearInterval(intervalID);
    }
    const id = setInterval(() => {
      setUserPosition((position) => {
        const diff = {
          x: clickWorldPosition.x - position.x,
          y: clickWorldPosition.y - position.y,
        };
        if (Math.abs(diff.x) < 3 && Math.abs(diff.y) < 3) {
          return clickWorldPosition;
        }
        const nextPosition = calcNewPosition(position, {
          x: diff.x / 10,
          y: diff.y / 10,
        });
        return nextPosition;
      });
    }, 1000 / 60);
    setIntervalID(id);
  };

  return (
    <div ref={stageRef}>
      <Stage // Field
        {...fieldSize}
        options={{ background: 0x1099bb }}
        className={props.className}
        onClick={onFieldClick}
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
          anchor={{ x: 0.5, y: 0.5 }}
        />
      </Stage>
    </div>
  );
};

export default Canvas;
