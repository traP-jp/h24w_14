import { atom } from "jotai";
import { Message } from "../model/message";

const fieldMessagesAtom = atom<Map<string, Message>>(new Map());

export default fieldMessagesAtom;
