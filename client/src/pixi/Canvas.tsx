import { Stage } from "@pixi/react";
import World from "./World";
import { useEffect, useRef, useState } from "react";
import {
  Position,
  DisplayPosition,
  displayPositionToPosition,
} from "./Position";
import Explorer from "./components/Explorer";

interface CanvasProps {
  className?: string;
}

const Canvas = (props: CanvasProps) => {
  const [userPosition, setUserPosition] = useState<Position | null>(null);
  const [fieldSize, setFieldSize] = useState<{
    width: number;
    height: number;
  } | null>(null);
  const [intervalID, setIntervalID] = useState<number | null>(null);
  const stageRef = useRef<HTMLDivElement>(null);

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

  if (fieldSize === null || userPosition === null) {
    return;
  }

  const userDisplayPosition = {
    left: fieldSize.width / 2,
    top: fieldSize.height / 2,
  };

  const calcNewPosition = (position: Position, diff: Position): Position => {
    const x = Math.max(Math.min(position.x + diff.x, 2000), 0);
    const y = Math.max(Math.min(position.y + diff.y, 2000), 0);
    return { x, y };
  };

  const updateUserPosition = (targetPosition: Position) => {
    setUserPosition((position) => {
      if (position === null) {
        return null;
      }
      const diff = {
        x: targetPosition.x - position.x,
        y: targetPosition.y - position.y,
      };
      if (Math.abs(diff.x) < 3 && Math.abs(diff.y) < 3) {
        return targetPosition;
      }
      const nextPosition = calcNewPosition(position, {
        x: diff.x / 10,
        y: diff.y / 10,
      });
      return nextPosition;
    });
  };

  const onFieldClick = (e: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
    if (stageRef.current === null) {
      return;
    }
    const stage = stageRef.current;
    const stageRect = stage.getBoundingClientRect();

    // クリックされた位置の、左画面の左上からの座標
    const clickDisplayPosition: DisplayPosition = {
      left: e.clientX - stageRect.left,
      top: e.clientY - stageRect.top,
    };

    // クリックされた場所に対応するワールド上の座標
    const clickPosition = displayPositionToPosition(
      clickDisplayPosition,
      userPosition,
      userDisplayPosition,
    );

    if (intervalID !== null) {
      clearInterval(intervalID);
    }
    const id = setInterval(() => {
      updateUserPosition(clickPosition);
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
          userDisplayPosition={userDisplayPosition}
        />
        <Explorer
          imgURL="https://q.trap.jp/api/v3/public/icon/ikura-hamu"
          displayPosition={userDisplayPosition}
        />
      </Stage>
    </div>
  );
};

export default Canvas;
