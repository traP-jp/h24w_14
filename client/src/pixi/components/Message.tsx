import { Container, Graphics, Sprite, Text } from "@pixi/react";
import React, { useRef, useState, useEffect, useCallback } from "react";
import { Graphics as PIXIGraphics, TextStyle } from "pixi.js";
import PIXI from "pixi.js";
import { Message as MessageModel } from "../../model/message";
import { themeColors } from "../theme";
import { traqIconURL } from "../../util/icon";

const messageIconSize = 30;

interface MessageBubbleProps {
  width: number;
  height: number;
}

const MessageBubble: React.FC<MessageBubbleProps> = (props) => {
  const draw = useCallback(
    (g: PIXIGraphics) => {
      g.clear();
      g.lineStyle(2, 0x000000);
      g.beginFill(themeColors.backgroundPrimary);
      g.drawRoundedRect(0, 0, props.width, props.height, 10);
      g.endFill();
    },
    [props],
  );
  return <Graphics draw={draw} />;
};

interface Props {
  expanded: boolean;
  message: MessageModel | null;
  collapse: () => void;
}

const userNameTextStyle = new TextStyle({ fontSize: 14, fill: "black" });
const messageTextStyle = new TextStyle({
  fontSize: 16,
  fill: "black",
  wordWrap: true,
  wordWrapWidth: 180,
  breakWords: true,
});

const Message: React.FC<Props> = ({ expanded, message, collapse }) => {
  const textRef = useRef<PIXI.Text>(null);
  const [bubbleSize, setBubbleSize] = useState({ width: 200, height: 100 });
  const user = {
    name: "ikura-hamu",
  };

  useEffect(() => {
    if (textRef.current) {
      const { width, height } = textRef.current;
      setBubbleSize({
        width: width + 20,
        height: height + 20,
      });
    }
  }, [message]);

  const iconImageSrc = traqIconURL(user?.name || "");
  if (!message || !user) return null;
  if (!expanded) return null;

  return (
    <Container {...message.position} interactive mouseout={collapse}>
      <Sprite
        image={iconImageSrc}
        x={0}
        y={0}
        anchor={0.5}
        width={messageIconSize}
        height={messageIconSize}
        interactive={true}
      />
      <Text
        text={user.name}
        x={messageIconSize / 2}
        y={messageIconSize / 2}
        anchor={{ x: 0, y: 1 }}
        style={userNameTextStyle}
      />
      <Container x={-messageIconSize / 2} y={messageIconSize / 2 + 8}>
        <MessageBubble width={bubbleSize.width} height={bubbleSize.height} />
        <Text
          ref={textRef}
          text={message.content}
          x={10}
          y={10}
          style={messageTextStyle}
        />
      </Container>
    </Container>
  );
};

export default Message;
