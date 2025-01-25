import { Container, Graphics, Sprite, Text } from "@pixi/react";
import messageIcon from "/src/assets/icons/messageIcon.svg";
import React, { useRef, useState, useEffect, useCallback } from "react";
import { Graphics as PIXIGraphics, TextStyle } from "pixi.js";
import PIXI from "pixi.js";
import { Position } from "../../Position";
import { themeColors } from "../theme";

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
  messageText: string;
  position: Position;
  user: {
    name: string;
    iconUrl: string;
  };
}

const userNameTextStyle = new TextStyle({ fontSize: 14, fill: "black" });
const messageTextStyle = new TextStyle({
  fontSize: 16,
  fill: "black",
  wordWrap: true,
  wordWrapWidth: 180,
  breakWords: true,
});

const Message: React.FC<Props> = ({ messageText, position, user }) => {
  const [showMessage, setShowMessage] = useState(false);
  const textRef = useRef<PIXI.Text>(null);
  const [bubbleSize, setBubbleSize] = useState({ width: 200, height: 100 });

  const handleMouseOver = useCallback(() => setShowMessage(true), []);
  const handleMouseOut = useCallback(() => setShowMessage(false), []);

  useEffect(() => {
    if (!showMessage) return;
    if (textRef.current) {
      const { width, height } = textRef.current;
      setBubbleSize({
        width: width + 20,
        height: height + 20,
      });
    }
  }, [showMessage]);

  const iconImageSrc = showMessage ? user.iconUrl : messageIcon;

  return (
    <Container
      {...position}
      interactive
      mouseout={handleMouseOut}
      mouseover={handleMouseOver}
    >
      <Sprite
        image={iconImageSrc}
        x={0}
        y={0}
        width={messageIconSize}
        height={messageIconSize}
        interactive={true}
      />
      {showMessage && (
        <>
          <Text
            text={user.name}
            x={messageIconSize + 10}
            y={messageIconSize - 10}
            anchor={{ x: 0, y: 1 }}
            style={userNameTextStyle}
          />
          <Container x={0} y={messageIconSize}>
            <MessageBubble
              width={bubbleSize.width}
              height={bubbleSize.height}
            />
            <Text
              ref={textRef}
              text={messageText}
              x={10}
              y={10}
              style={messageTextStyle}
            />
          </Container>
        </>
      )}
    </Container>
  );
};

export default Message;
