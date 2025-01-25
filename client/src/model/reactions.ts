import Iine from "../assets/reactions/iine.webp";
import Kusa from "../assets/reactions/kusa.webp";
import Pro from "../assets/reactions/pro.webp";
import { Position } from "./position";

export type ReactionName = "iine" | "kusa" | "pro";

export const ReactionAssets = {
  iine: Iine,
  kusa: Kusa,
  pro: Pro,
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
