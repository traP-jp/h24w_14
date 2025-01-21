import { Sprite } from "@pixi/react";
import { DisplayPosition } from "../Position";

export interface Props {
  imgURL: string;
  displayPosition: DisplayPosition;
}

const Explorer: React.FC<Props> = ({ displayPosition, imgURL }) => {
  return (
    <Sprite
      image={imgURL}
      x={displayPosition.left}
      y={displayPosition.top}
      anchor={{ x: 0.5, y: 0.5 }}
      width={50}
      height={50}
    />
  );
};

export default Explorer;
