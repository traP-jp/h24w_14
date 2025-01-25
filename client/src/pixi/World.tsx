import { Container } from "@pixi/react";
import Rectangle from "./components/Rectangle";
import "@pixi/events";
import { DisplayPosition, Position } from "../model/position";
import React from "react";
import Message from "./components/Message";
import SpeakerPhone from "./components/SpeakerPhone";
import Reaction from "./components/Reaction";
import { traqIconURL } from "../util/icon";
import { useAtomValue } from "jotai";
import messagesAtom from "../state/message";
import MessageIcon from "./components/MessageIcon";
import useMessageExpanded from "./hooks/message";
import { isInsideField } from "../util/field";

interface Props {
  userPosition: Position;
  userDisplayPosition: DisplayPosition;
  fieldSize: { width: number; height: number };
}

const World: React.FC<Props> = ({
  userPosition,
  userDisplayPosition,
  fieldSize,
}) => {
  const { expanded, collapseMessage, expandMessage, message } =
    useMessageExpanded();
  const messages = useAtomValue(messagesAtom);
  const messageNodes = [];
  for (const message of messages.values()) {
    if (!isInsideField(message.position, fieldSize, userPosition)) {
      continue;
    }
    messageNodes.push(
      <MessageIcon
        currentExpandedMessageId={message.id}
        expander={expandMessage}
        key={message.id}
        message={message}
      />,
    );
  }

  //TODO: モック用なので後で消す
  for (let i = 1; i <= 3; i++) {
    messageNodes.push(
      <MessageIcon
        currentExpandedMessageId={message?.id}
        expander={expandMessage}
        key={i}
        message={{
          id: i.toString(),
          position: { x: 100 * i + 10, y: 100 * i },
          userId: "ikura-hamu",
          content: "Hello, World!".repeat(i * 5),
          createdAt: new Date(),
          updatedAt: new Date(),
          expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000), // 1 day later
        }}
      />,
    );
  }

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
      <SpeakerPhone
        position={{ x: 1700, y: 1700 }}
        name="#gps/times/ikura-hamu"
        radius={100}
      />
      <SpeakerPhone
        position={{ x: 200, y: 200 }}
        name="#gps/times/ikura-hamu"
        radius={100}
      />

      {messageNodes}
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
      <Message
        expanded={expanded}
        message={message}
        collapse={collapseMessage}
      />
    </Container>
  );
};

export default World;
