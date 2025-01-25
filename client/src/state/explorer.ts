import { atom } from "jotai";
import Explorer from "../model/explorer";
import { Position } from "../model/position";

type ExplorersMap = Map<string, Explorer & { previousPosition?: Position }>;
const fieldExplorersAtom = atom<ExplorersMap>(new Map());
export default fieldExplorersAtom;
