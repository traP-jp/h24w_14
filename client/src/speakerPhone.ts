import { Position } from "./Position";

interface SpeakerPhone {
  id: string;
  position: Position;
  receiveRange: number;
  name: string;
  createdAt: Date;
  updatedAt: Date;
}

export default SpeakerPhone;
