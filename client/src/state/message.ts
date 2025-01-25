import { atom } from "jotai";
import { Message } from "../model/message";

const fieldMessagesAtom = atom<Message[]>([]);

export default fieldMessagesAtom;
