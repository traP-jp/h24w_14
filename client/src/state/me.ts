import { atom } from "jotai";
import User from "../model/user";

const meAtom = atom<User | undefined>(undefined);
export default meAtom;
