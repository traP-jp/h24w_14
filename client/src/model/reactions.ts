import Iine from "../assets/reactions/iine.webp";
import Kusa from "../assets/reactions/kusa.webp";
import Pro from "../assets/reactions/pro.webp";
import Fire from "../assets/reactions/fire.png";
import Smile from "../assets/reactions/smile.png";

import { Position } from "./position";

export type ReactionName = "iine" | "kusa" | "pro" | "fire" | "smile";

export const ReactionAssets = {
  iine: Iine,
  kusa: Kusa,
  pro: Pro,
  fire: Fire,
  smile: Smile,
};

interface Reaction {
  id: string;
  userId: string;
  position: Position;
  kind: ReactionName;
  createdAt: Date;
  expiresAt: Date;
}

export default Reaction;
