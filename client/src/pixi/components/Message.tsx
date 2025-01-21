import { Sprite } from "@pixi/react";
import messageIcon from "/src/assets/icons/messageIcon.svg";

const messageIconSize = 30;

const Message: React.FC = () => {
  return (
    <Sprite
      image={messageIcon}
      x={100}
      y={0}
      width={messageIconSize}
      height={messageIconSize}
    />
  );
};

export default Message;
