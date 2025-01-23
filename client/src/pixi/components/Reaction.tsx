import { Container, Sprite, Text } from "@pixi/react";
import React, { useEffect, useRef, useState } from "react";
import { ReactionAssets, ReactionName } from "../reactions";
import { Position } from "../Position";
import { themeColors } from "../theme";
import Bubble from "./Bubble";
import PIXI, { TextStyle } from "pixi.js";

interface Props {
  position: Position;
  reaction: ReactionName;
  user: {
    name: string;
    iconURL: string;
  };
}

const reactionImageSize = 25;
const userIconSize = 14;

const Reaction: React.FC<Props> = ({ position, reaction, user }) => {
  const [showUser, setShowUser] = useState(false);
  const iconAndNameRef = useRef<PIXI.Container>(null);
  const [iconAndNameWidth, setIconAndNameWidth] = useState(0);

  useEffect(() => {
    if (!showUser) {
      return;
    }
    if (iconAndNameRef.current) {
      setIconAndNameWidth(iconAndNameRef.current.width);
    }
  }, [showUser, iconAndNameRef]);

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
        <>
          <Bubble
            x={0}
            y={-reactionImageSize / 2 - 5}
            radius={10}
            color={themeColors.accentSecondary}
            height={20}
            width={iconAndNameWidth + 10}
            fillColor={"#000000"}
            lineWidth={2}
          />
          <Container
            x={-iconAndNameWidth / 2}
            y={-reactionImageSize / 2 - 15}
            ref={iconAndNameRef}
          >
            <Sprite
              image={user.iconURL}
              width={userIconSize}
              height={userIconSize}
              anchor={{ x: 0, y: 0.5 }}
            />
            <Text
              anchor={{ x: 0, y: 0.5 }}
              text={user.name}
              x={userIconSize + 5}
              y={0}
              style={
                new TextStyle({
                  fill: themeColors.textSecondary,
                  fontSize: 14,
                  fontWeight: "bold",
                })
              }
            />
          </Container>
        </>
      )}
    </Container>
  );
};

export default Reaction;
