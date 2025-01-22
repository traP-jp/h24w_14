import { Container, Sprite } from "@pixi/react";
import Rectangle from "./components/Rectangle";
import "@pixi/events";
import { DisplayPosition, Position } from "./Position";
import React from "react";
import Message from "./components/Message";

interface Props {
  userPosition: Position;
  userDisplayPosition: DisplayPosition;
}

const World: React.FC<Props> = ({ userPosition, userDisplayPosition }) => {
  return (
    <Container
      width={2000}
      height={2000}
      x={-userPosition.x + userDisplayPosition.left}
      y={-userPosition.y + userDisplayPosition.top}
      anchor={{ x: 0, y: 0 }}
      interactive={true}
    >
      <Rectangle
        lineWidth={2}
        color={0xffffff}
        width={2000}
        height={2000}
        fillColor={0xeeeeee}
        fillAlpha={1}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={0}
        y={0}
        width={100}
        height={100}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={0}
        y={1900}
        width={100}
        height={100}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={1900}
        y={0}
        width={100}
        height={100}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={1900}
        y={1900}
        width={100}
        height={100}
      />
      <Message
        messageText={"メッセージ".repeat(20)}
        displayPosition={{ left: 100, top: 100 }}
        user={{
          name: "ikura-hamu",
          iconUrl: "https://q.trap.jp/api/v3/public/icon/ikura-hamu",
        }}
      />
    </Container>
  );
};

export default World;
