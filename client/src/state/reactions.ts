import { atom } from "jotai";

import Reaction from "../reactions";

const fieldReactionsAtom = atom<Reaction[]>([]);
export default fieldReactionsAtom;
