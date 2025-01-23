import { Container, Sprite } from "@pixi/react";
import React, { useState } from "react";
import { ReactionAssets, ReactionName } from "../reactions";
import { Position } from "../Position";
import Circle from "./Circle";
import { themeColors } from "../theme";

interface Props {
  position: Position;
  reaction: ReactionName;
  user: {
    name: string;
    iconURL: string;
  };
}

const reactionImageSize = 25;
const userIconSize = 20;

const Reaction: React.FC<Props> = ({ position, reaction, user }) => {
  const [showUser, setShowUser] = useState(false);

  return (
    <Container
      {...position}
      interactive
      onmouseover={() => setShowUser(true)}
      onmouseout={() => setShowUser(false)}
    >
      <Sprite
        image={ReactionAssets[reaction]}
        x={0}
        y={0}
        anchor={{ x: 0.5, y: 0.5 }}
        width={reactionImageSize}
        height={reactionImageSize}
      />
      {showUser && (
        <Container anchor={0.5} x={reactionImageSize} y={reactionImageSize}>
          <Circle
            x={0}
            y={0}
            lineWidth={1}
            color={themeColors.accentPrimary}
            fillColor={themeColors.backgroundPrimary}
            radius={(userIconSize / 2) * 1.5}
          />
          <Sprite
            anchor={0.5}
            image={user.iconURL}
            width={userIconSize}
            height={userIconSize}
          />
        </Container>
      )}
    </Container>
  );
};

export default Reaction;
