import Abao_iyaaaa from "../assets/reactions/abao_iyaaaa.png";
import Eyes from "../assets/reactions/eyes.png";
import Fire from "../assets/reactions/fire.png";
import Hetareneko_iyaaa from "../assets/reactions/hetareneko_iyaaa.png";
import Iine from "../assets/reactions/iine.webp";
import Kusa from "../assets/reactions/kusa.webp";
import Pro from "../assets/reactions/pro.webp";
import Smile from "../assets/reactions/smile.png";

import { Position } from "./position";

export type ReactionName =
  | "iine"
  | "kusa"
  | "pro"
  | "fire"
  | "smile"
  | "hetareneko_iyaaa"
  | "abao_iyaaaa"
  | "eyes";

export const ReactionAssets = {
  iine: Iine,
  kusa: Kusa,
  pro: Pro,
  hetareneko_iyaaa: Hetareneko_iyaaa,
  abao_iyaaaa: Abao_iyaaaa,
  eyes: Eyes,
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
