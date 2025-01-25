import { Position } from "./Position";

export interface Message {
  id: string;
  userId: string;
  position: Position;
  content: string;
  createdAt: Date;
  updatedAt: Date;
  expiresAt: Date;
}
