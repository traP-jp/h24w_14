import { Sprite } from "@pixi/react";
import { DisplayPosition } from "../Position";

export interface Props {
  imgURL: string;
  displayPosition: DisplayPosition;
}

const Explorer = (props: Props) => {
  const { displayPosition, imgURL } = props;
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
