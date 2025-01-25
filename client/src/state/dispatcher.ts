import { atom } from "jotai";
import { ExplorerMessageDispatcher } from "../api/explorer";

const dispatcherAtom = atom<ExplorerMessageDispatcher | undefined>(undefined);

export default dispatcherAtom;
