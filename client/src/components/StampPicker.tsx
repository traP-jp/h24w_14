import { SmileOutlined } from "@ant-design/icons";
import { Button, Popover } from "antd";
import React, { useState } from "react";
import Reaction, { ReactionAssets } from "../model/reactions";

export const StampPicker: React.FC = () => {
  const [isPopoverOpen, setIsPopoverOpen] = useState(false);
  const reactions: Reaction[] = Array(8)
    .fill([
      {
        id: "1",
        userId: "1",
        position: { x: 0, y: 0 },
        kind: "iine",
        createdAt: new Date(),
        expiresAt: new Date(),
      },
      {
        id: "2",
        userId: "2",
        position: { x: 0, y: 0 },
        kind: "kusa",
        createdAt: new Date(),
        expiresAt: new Date(),
      },
      {
        id: "3",
        userId: "3",
        position: { x: 0, y: 0 },
        kind: "pro",
        createdAt: new Date(),
        expiresAt: new Date(),
      },
    ])
    .flat();

  return (
    <Popover
      content={
        <div className="grid grid-cols-4 gap-2">
          {reactions.map((reaction, index) => (
            <Button
              key={index}
              type="text"
              icon={
                <img src={ReactionAssets[reaction.kind]} alt={reaction.kind} />
              }
              className="hover:bg-background-secondary"
            />
          ))}
        </div>
      }
      trigger="click"
      open={isPopoverOpen}
      onOpenChange={setIsPopoverOpen}
    >
      <Button icon={<SmileOutlined />} />
    </Popover>
  );
};
