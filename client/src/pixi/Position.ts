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
  userDisplayPosition: DisplayPosition,
): Position => {
  return {
    x: displayPosition.left + userPosition.x - userDisplayPosition.left,
    y: displayPosition.top + userPosition.y - userDisplayPosition.top,
  };
};
