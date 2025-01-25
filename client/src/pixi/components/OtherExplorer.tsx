import { Position } from "../../model/position";
import ExplorerModel from "../../model/explorer";
import Explorer from "./Explorer";
import { useUser } from "../../api/user";
import { traqIconURL } from "../../util/icon";
import { useTick } from "@pixi/react";
import { useState } from "react";

interface Props {
  explorer: ExplorerModel;
  previousPosition?: Position;
}

const OtherExplorer: React.FC<Props> = ({ explorer, previousPosition }) => {
  // previousPosition: 1つ前の受信時の位置
  const { position: targetPosition, userId } = explorer; // 目標の位置
  const { data, error, isLoading } = useUser(userId);
  const [position, setPosition] = useState(targetPosition); // 実際の現在の位置

  useTick(() => {
    if (previousPosition) {
      if (position.x === targetPosition.x && position.y === targetPosition.y) {
        return;
      }
      setPosition((pos) => {
        const diff = {
          x: targetPosition.x - pos.x,
          y: targetPosition.y - pos.y,
        };
        if (Math.abs(diff.x) < 3 && Math.abs(diff.y) < 3) {
          return targetPosition;
        }
        const speed = 0.1;
        return {
          x: position.x + diff.x * speed,
          y: position.y + diff.y * speed,
        };
      });
    }
  });

  if (isLoading || error || !data) {
    return null;
  }
  const user = data.user;
  if (!user) {
    return null;
  }

  return (
    <Explorer
      position={position}
      imgURL={traqIconURL(user.name)}
      isMe={false}
      name={user.name}
    />
  );
};

export default OtherExplorer;
