import SpeakerPhone from "../model/speakerPhone";
import { atom } from "jotai";

const fieldSpeakerPhonesAtom = atom<SpeakerPhone[]>([]);
export default fieldSpeakerPhonesAtom;
