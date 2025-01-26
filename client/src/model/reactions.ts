import Abao_iyaaaa from "../assets/reactions/Abao_iyaaaa.webp";
import Eyes from "../assets/reactions/eyes.webp";
import Hetareneko_iyaaa from "../assets/reactions/Hetareneko_iyaaa.webp";
import Iine from "../assets/reactions/iine.webp";
import Kusa from "../assets/reactions/kusa.webp";
import Pro from "../assets/reactions/pro.webp";
import { Position } from "./position";

export type ReactionName = "iine" | "kusa" | "pro";

export const ReactionAssets = {
  iine: Iine,
  kusa: Kusa,
  pro: Pro,
  hetareneko_iyaaa: Hetareneko_iyaaa,
  abao_iyaaaa: Abao_iyaaaa,
  eyes: Eyes,
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
