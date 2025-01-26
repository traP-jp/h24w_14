import { Container } from "@pixi/react";
import Rectangle from "./components/Rectangle";
import "@pixi/events";
import { DisplayPosition, Position } from "../model/position";
import React, { useMemo } from "react";
import Message from "./components/Message";
import SpeakerPhone from "./components/SpeakerPhone";
import Reaction from "./components/Reaction";
import { traqIconURL } from "../util/icon";
import { useAtomValue } from "jotai";
import messagesAtom from "../state/message";
import MessageIcon from "./components/MessageIcon";
import useMessageExpanded from "./hooks/message";
import { isInsideField } from "../util/field";
import speakerPhonesAtom from "../state/speakerPhone";
import reactionsAtom from "../state/reactions";
import explorersAtom from "../state/explorer";
import OtherExplorer from "./components/OtherExplorer";

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
  const {
    expanded,
    collapseMessage,
    expandMessage,
    message: expandedMessage,
  } = useMessageExpanded();
  const messages = useAtomValue(messagesAtom);
  const speakerPhones = useAtomValue(speakerPhonesAtom);
  const reactions = useAtomValue(reactionsAtom);
  const explorers = useAtomValue(explorersAtom);

  const messageNodes = useMemo(() => {
    return Array.from(messages.values())
      .filter((message) =>
        isInsideField(message.position, fieldSize, userPosition),
      )
      .map((message) => {
        return (
          <MessageIcon
            currentExpandedMessageId={expandedMessage?.id}
            expander={expandMessage}
            key={message.id}
            message={message}
          />
        );
      });
  }, [messages, fieldSize, userPosition, expandedMessage?.id, expandMessage]);

  const speakerPhoneNodes = useMemo(() => {
    return Array.from(speakerPhones.values())
      .filter((speakerPhone) =>
        isInsideField(speakerPhone.position, fieldSize, userPosition),
      )
      .map((speakerPhone) => {
        return (
          <SpeakerPhone
            key={speakerPhone.name}
            position={speakerPhone.position}
            name={speakerPhone.name}
            radius={150}
          />
        );
      });
  }, [speakerPhones, fieldSize, userPosition]);

  const reactionsNodes = useMemo(() => {
    return Array.from(reactions.values())
      .filter((reaction) =>
        isInsideField(reaction.position, fieldSize, userPosition),
      )
      .map((reaction) => (
        <Reaction
          key={reaction.id}
          position={reaction.position}
          reaction={reaction.kind}
          user={{
            name: reaction.userId,
            iconURL: traqIconURL(reaction.userId),
          }}
          userId={reaction.userId}
        />
      ));
  }, [fieldSize, reactions, userPosition]);

  const explorerNodes = useMemo(() => {
    return Array.from(explorers.values()).map((explorer) => {
      return (
        <OtherExplorer
          key={explorer.id}
          explorer={explorer}
          previousPosition={explorer.previousPosition}
        />
      );
    });
  }, [explorers]);

  //TODO: モック用なので後で消す
  // {
  // for (let i = 1; i <= 3; i++) {
  //   messageNodes.push(
  //     <MessageIcon
  //       currentExpandedMessageId={message?.id}
  //       expander={expandMessage}
  //       key={i}
  //       message={{
  //         id: i.toString(),
  //         position: { x: 100 * i + 10, y: 100 * i },
  //         userId: "ikura-hamu",
  //         content: "Hello, World!".repeat(i * 5),
  //         createdAt: new Date(),
  //         updatedAt: new Date(),
  //         expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000), // 1 day later
  //       }}
  //     />,
  //   );
  // }
  //   for (let i = 4; i <= 6; i++) {
  //     speakerPhoneNodes.push(
  //       <SpeakerPhone
  //         key={i}
  //         position={{ x: 100 * i + 10, y: 100 * i }}
  //         name="SpeakerPhone"
  //         radius={100}
  //       />,
  //     );
  //   }
  //   for (let i = 7; i <= 9; i++) {
  //     reactionsNodes.push(
  //       <Reaction
  //         key={i}
  //         position={{ x: 100 * i + 10, y: 100 * i }}
  //         reaction="iine"
  //         user={{
  //           name: "ikura-hamu",
  //           iconURL: traqIconURL("ikura-hamu"),
  //         }}
  //       />,
  //     );
  //   }
  // }

  return (
    <Container
      width={5000}
      height={5000}
      x={-userPosition.x + userDisplayPosition.left}
      y={-userPosition.y + userDisplayPosition.top}
      anchor={{ x: 0, y: 0 }}
      interactive={true}
    >
      <Rectangle
        lineWidth={2}
        color={0xffffff}
        width={5000}
        height={5000}
        fillColor={0xeeeeee}
        fillAlpha={1}
      />
      {speakerPhoneNodes}
      {messageNodes}
      {explorerNodes}
      {reactionsNodes}
      <Message
        expanded={expanded}
        message={expandedMessage}
        collapse={collapseMessage}
      />
    </Container>
  );
};

export default World;
