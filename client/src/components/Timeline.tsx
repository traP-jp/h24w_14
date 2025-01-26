import { Avatar, List } from "antd";
import React from "react";
import { InputMessage } from "./InputMessage";
import { useAtomValue } from "jotai";
import fieldMessagesAtom from "../state/message";
import { isInsideField } from "../util/field";
import fieldSizeAtom from "../state/field";
import userPositionAtom from "../state/userPosition";
import { Message } from "../model/message";
import { useUser } from "../api/user";
import { traqIconURL } from "../util/icon";

interface TimelineItemProps {
  message: Message;
}

const TimelineItem: React.FC<TimelineItemProps> = ({ message }) => {
  const { data, error, isLoading } = useUser(message.userId);
  if (isLoading || error || !data) {
    return null;
  }

  return (
    <List.Item key={message.id}>
      <List.Item.Meta
        avatar={<Avatar src={traqIconURL(data.user?.name ?? "")} />}
        title={data.user?.displayName}
        description={data.user?.name}
      />
      {message.content}
    </List.Item>
  );
};

export const Timeline: React.FC = () => {
  const messagesMap = useAtomValue(fieldMessagesAtom);
  const fieldSize = useAtomValue(fieldSizeAtom);
  const mePosition = useAtomValue(userPositionAtom);

  const messages = Array.from(messagesMap.values()).filter((message) => {
    if (!fieldSize || !mePosition) {
      return false;
    }
    return isInsideField(message.position, fieldSize, mePosition);
  });

  return (
    <div className="flex flex-col h-screen w-full bg-background-primary">
      <div id="scrollableDiv" className="size-full overflow-auto px-4">
        <List
          dataSource={messages}
          renderItem={(message) => (
            <TimelineItem message={message} key={message.id} />
          )}
        />
      </div>
      <InputMessage />
    </div>
  );
};
