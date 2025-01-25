import React from "react";
import { Message } from "../../model/message";
import { Sprite } from "@pixi/react";
import messageIcon from "../../assets/icons/messageIcon.svg";

interface Props {
  message: Message;
  expander: (message: Message) => void;
  currentExpandedMessageId: string | undefined;
}

const messageIconSize = 30;

const ExpandedIcon: React.FC<Props> = ({
  message,
  expander,
  currentExpandedMessageId,
}) => {
  if (currentExpandedMessageId === message.id) {
    return null;
  }
  return (
    <>
      <Sprite
        image={messageIcon}
        x={message.position.x}
        y={message.position.y}
        anchor={0.5}
        width={messageIconSize}
        height={messageIconSize}
        interactive={true}
        onmouseover={() => {
          expander(message);
        }}
      />
    </>
  );
};

export default ExpandedIcon;
