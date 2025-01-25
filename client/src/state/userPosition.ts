import { atom } from "jotai";
import type { Position } from "../model/position";

const userPositionAtom = atom<Position | null>(null);
export default userPositionAtom;
