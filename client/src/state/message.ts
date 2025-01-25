import { atom } from "jotai";
import { Message } from "../message";

const fieldMessagesAtom = atom<Message[]>([]);

export default fieldMessagesAtom;
