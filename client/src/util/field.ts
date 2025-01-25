import { Position } from "../model/position";

const isInsideField = (
  targetPosition: Position,
  fieldSize: { width: number; height: number },
  userPosition: Position,
) => {
  const xDiff = Math.abs(targetPosition.x - userPosition.x);
  const yDiff = Math.abs(targetPosition.y - userPosition.y);
  return xDiff <= fieldSize.width / 2 && yDiff <= fieldSize.height / 2;
};

export { isInsideField };
