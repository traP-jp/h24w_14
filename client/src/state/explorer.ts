import { atom } from "jotai";
import Explorer from "../explorer";
import { Position } from "../Position";

type ExplorersMap = Map<string, Explorer & { previousPosition?: Position }>;
const fieldExplorersAtom = atom<ExplorersMap>(new Map());
export default fieldExplorersAtom;
