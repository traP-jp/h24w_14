import { Position } from "./position";

interface SpeakerPhone {
  id: string;
  position: Position;
  receiveRange: number;
  name: string;
  createdAt: Date;
  updatedAt: Date;
}

export default SpeakerPhone;
