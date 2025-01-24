import { Sprite, Text, Graphics } from "@pixi/react";
import { DisplayPosition, Position } from "../Position";
import { TextStyle } from "pixi.js";
import { themeColors } from "../theme";

export type Props =
  | {
      name: string;
      imgURL: string;
      isMe: true;
      position: DisplayPosition;
    }
  | {
      name: string;
      imgURL: string;
      isMe: false;
      position: Position;
    };

const iconSize = 50;

const Explorer: React.FC<Props> = ({ position, imgURL, isMe, name }) => {
  const pos = isMe ? { x: position.left, y: position.top } : position;

  return (
    <>
      {isMe && (
        <Graphics
          draw={(g) => {
            g.clear();
            g.lineStyle(1, themeColors.textSecondary);
            g.beginFill(themeColors.backgroundSecondary);
            g.drawRoundedRect(
              pos.x - iconSize / 2,
              pos.y - iconSize / 2,
              iconSize,
              iconSize,
              0,
            );
          }}
        />
      )}
      <Sprite
        image={imgURL}
        {...pos}
        anchor={{ x: 0.5, y: 0.5 }}
        width={iconSize}
        height={iconSize}
      />
      <Text
        text={name}
        x={pos.x}
        y={pos.y - iconSize / 2 - 10}
        anchor={{ x: 0.5, y: 0.5 }}
        style={
          new TextStyle({
            fontSize: 12,
            fill: themeColors.textSecondary,
          })
        }
      />
    </>
  );
};

export default Explorer;
