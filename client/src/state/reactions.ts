import { atom } from "jotai";

import Reaction from "../model/reactions";

const fieldReactionsAtom = atom<Reaction[]>([]);
export default fieldReactionsAtom;
