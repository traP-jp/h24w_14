import { Stage } from "@pixi/react";
import World from "./World";
import React, { useCallback, useEffect, useMemo, useRef } from "react";
import {
  Position,
  DisplayPosition,
  displayPositionToPosition,
} from "../model/position";
import Explorer from "./components/Explorer";
import PIXI from "pixi.js";
import { useAtom, useAtomValue } from "jotai";
import dispatcherAtom from "../state/dispatcher";
import userPositionAtom from "../state/userPosition";
import meAtom from "../state/me";
import { traqIconURL } from "../util/icon";
import fieldSizeAtom from "../state/field";

const mountHandler = import.meta.env.DEV
  ? (app: PIXI.Application) => {
      // settings for pixi.js devtool https://github.com/bfanger/pixi-inspector
      (globalThis as any).__PIXI_APP__ = app; // eslint-disable-line
    }
  : undefined;

interface Props {
  className?: string;
}

const calcNewPosition = (position: Position, diff: Position): Position => {
  const x = Math.max(Math.min(position.x + diff.x, 2000), 0);
  const y = Math.max(Math.min(position.y + diff.y, 2000), 0);
  return { x, y };
};

const Canvas: React.FC<Props> = (props) => {
  const [userPosition, setUserPosition] = useAtom(userPositionAtom);
  const [fieldSize, setFieldSize] = useAtom(fieldSizeAtom);
  const intervalID = useRef<number | null>(null);
  const stageRef = useRef<HTMLDivElement>(null);
  const dispatcher = useAtomValue(dispatcherAtom);
  const me = useAtomValue(meAtom);

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
  }, [setFieldSize, setUserPosition]);

  const userDisplayPosition = useMemo(() => {
    if (fieldSize === null) {
      return null;
    }
    return {
      left: fieldSize.width / 2,
      top: fieldSize.height / 2,
    };
  }, [fieldSize]);

  const updateUserPosition = useCallback(
    (targetPosition: Position) => {
      setUserPosition((position) => {
        if (position === null || fieldSize === null) {
          return null;
        }
        const diff = {
          x: targetPosition.x - position.x,
          y: targetPosition.y - position.y,
        };
        if (Math.abs(diff.x) < 3 && Math.abs(diff.y) < 3) {
          dispatcher?.({
            position: targetPosition,
            size: fieldSize,
          });

          clearInterval(intervalID.current ?? undefined);
          return targetPosition;
        }
        const nextPosition = calcNewPosition(position, {
          x: diff.x / 10,
          y: diff.y / 10,
        });
        dispatcher?.({
          position: nextPosition,
          size: fieldSize,
        });
        return nextPosition;
      });
    },
    [dispatcher, fieldSize, setUserPosition],
  );

  const onFieldClick = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
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

      if (userPosition === null || userDisplayPosition === null) {
        return;
      }

      // クリックされた場所に対応するワールド上の座標
      const clickPosition = displayPositionToPosition(
        clickDisplayPosition,
        userPosition,
        userDisplayPosition,
      );

      if (intervalID.current !== null) {
        clearInterval(intervalID.current);
      }

      const id = setInterval(() => {
        updateUserPosition(clickPosition);
      }, 1000 / 60);
      intervalID.current = id;
    },
    [updateUserPosition, userDisplayPosition, userPosition],
  );

  if (
    fieldSize === null ||
    userPosition === null ||
    userDisplayPosition === null
  ) {
    return;
  }

  return (
    <div ref={stageRef}>
      <Stage // Field
        {...fieldSize}
        options={{ background: 0x1099bb }}
        className={props.className}
        onClick={onFieldClick}
        onMount={mountHandler}
      >
        <World
          fieldSize={fieldSize}
          userPosition={userPosition}
          userDisplayPosition={userDisplayPosition}
        />
        <Explorer
          imgURL={traqIconURL(me?.name ?? "")}
          position={userDisplayPosition}
          isMe
          name={me?.name ?? ""}
        />
      </Stage>
    </div>
  );
};

export default Canvas;
