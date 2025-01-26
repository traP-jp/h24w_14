import { SmileOutlined } from "@ant-design/icons";
import { Button, Popover } from "antd";
import { useAtomValue } from "jotai";
import React, { useState } from "react";
import { useCreateReaction } from "../api/reaction";
import { ReactionAssets, ReactionName } from "../model/reactions";
import { roundedUserPositionAtom } from "../state/userPosition";

export const ReactionPicker: React.FC = () => {
  const [isPopoverOpen, setIsPopoverOpen] = useState(false);
  const reactionNames = Object.keys(ReactionAssets) as ReactionName[];
  const clickHandler = async (reactionName: ReactionName) => {
    await trigger({ position: position ?? undefined, kind: reactionName });
  };
  const position = useAtomValue(roundedUserPositionAtom);
  const { trigger } = useCreateReaction();

  return (
    <Popover
      content={
        <div className="grid grid-cols-4 gap-2">
          {reactionNames.map((reactionName, index) => (
            <Button
              key={index}
              type="text"
              icon={
                <img src={ReactionAssets[reactionName]} alt={reactionName} />
              }
              className="hover:bg-background-secondary"
              onClick={() => {
                clickHandler(reactionName);
              }}
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
