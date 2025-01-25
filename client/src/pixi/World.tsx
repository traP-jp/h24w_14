import { Container, Sprite } from "@pixi/react";
import Rectangle from "./components/Rectangle";
import "@pixi/events";
import { DisplayPosition, Position } from "../model/position";
import React from "react";
import Message from "./components/Message";
import SpeakerPhone from "./components/SpeakerPhone";
import Reaction from "./components/Reaction";
import { traqIconURL } from "../util/icon";

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
        image={traqIconURL("ikura-hamu")}
        x={0}
        y={0}
        width={100}
        height={100}
      />
      <Sprite
        image={traqIconURL("ikura-hamu")}
        x={0}
        y={1900}
        width={100}
        height={100}
      />
      <Sprite
        image={traqIconURL("ikura-hamu")}
        x={1900}
        y={0}
        width={100}
        height={100}
      />
      <Sprite
        image={traqIconURL("ikura-hamu")}
        x={1900}
        y={1900}
        width={100}
        height={100}
      />
      <Message
        messageText={"メッセージ".repeat(20)}
        position={{ x: 100, y: 100 }}
        user={{
          name: "ikura-hamu",
          iconUrl: traqIconURL("ikura-hamu"),
        }}
      />
      <Message
        messageText={"メッセージ".repeat(20)}
        position={{ x: 1800, y: 1800 }}
        user={{
          name: "ikura-hamu",
          iconUrl: traqIconURL("ikura-hamu"),
        }}
      />
      <SpeakerPhone
        position={{ x: 200, y: 200 }}
        name="#gps/times/ikura-hamu"
        radius={100}
      />
      <SpeakerPhone
        position={{ x: 1700, y: 1700 }}
        name="#gps/times/ikura-hamu"
        radius={100}
      />
      <Reaction
        position={{ x: 300, y: 300 }}
        reaction="kusa"
        user={{
          name: "SSlime",
          iconURL: traqIconURL("SSlime"),
        }}
      />
      <Reaction
        position={{ x: 200, y: 500 }}
        reaction="iine"
        user={{
          name: "Ras",
          iconURL: traqIconURL("Ras"),
        }}
      />
      <Reaction
        position={{ x: 250, y: 500 }}
        reaction="pro"
        user={{
          name: "H1rono_K",
          iconURL: traqIconURL("H1rono_K"),
        }}
      />
    </Container>
  );
};

export default World;
