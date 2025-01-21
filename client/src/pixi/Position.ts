export interface Position {
  x: number;
  y: number;
}

export interface DisplayPosition {
  left: number;
  top: number;
}

export const displayPositionToPosition = (
  displayPosition: DisplayPosition,
  userPosition: Position,
  userDisplayPosition: DisplayPosition
): Position => {
  return {
    x: dispayPosition.left + userPosition.x - userDisplayPosition.left,
    y: dispayPosition.top + userPosition.y - userDisplayPosition.top,
  };
};
