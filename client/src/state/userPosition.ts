import { atom } from "jotai";
import type { Position } from "../model/position";

const userPositionAtom = atom<Position | null>(null);
export const roundedUserPositionAtom = atom<Position | null>((get) => {
  const position = get(userPositionAtom);
  if (position === null) {
    return null;
  }
  return {
    x: Math.round(position.x),
    y: Math.round(position.y),
  };
});
export default userPositionAtom;
