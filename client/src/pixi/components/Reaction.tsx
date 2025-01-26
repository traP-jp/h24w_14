import { Container, Sprite, Text } from "@pixi/react";
import React, { useState } from "react";
import { ReactionAssets, ReactionName } from "../../model/reactions";
import { Position } from "../../model/position";
import Bubble from "./Bubble";
import { TextStyle } from "pixi.js";
import { useUser } from "../../api/user";
import { traqIconURL } from "../../util/icon";

interface Props {
  position: Position;
  reaction: ReactionName;
  user: {
    name: string;
    iconURL: string;
  };
  userId: string;
}

const reactionImageSize = 25;
const userIconSize = 14;

const Reaction: React.FC<Props> = ({ position, reaction, userId }) => {
  const [showUser, setShowUser] = useState(false);
  const { data } = useUser(userId);
  const user = data?.user;
  if (!user) {
    return null;
  }

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
            y={-reactionImageSize / 2}
            padding={2.5}
            radius={10}
            color={"#000000"}
            fillColor={"#000000"}
            lineWidth={2}
          >
            <Sprite
              image={traqIconURL(user.name)}
              width={userIconSize}
              height={userIconSize}
            />
            <Text
              text={user.name}
              x={userIconSize + 5}
              y={0}
              style={
                new TextStyle({
                  fill: "#ffffff",
                  fontSize: 14,
                  fontWeight: "bold",
                })
              }
            />
          </Bubble>
        </>
      )}
    </Container>
  );
};

export default Reaction;
