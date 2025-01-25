import SpeakerPhone from "../speakerPhone";
import { atom } from "jotai";

const fieldSpeakerPhonesAtom = atom<SpeakerPhone[]>([]);
export default fieldSpeakerPhonesAtom;
