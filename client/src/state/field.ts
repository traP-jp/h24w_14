import { atom } from "jotai";

const fieldSizeAtom = atom<{ width: number; height: number } | null>(null);
export default fieldSizeAtom;
