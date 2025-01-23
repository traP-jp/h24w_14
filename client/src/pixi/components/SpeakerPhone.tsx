import React, { useState } from "react";
import speakerPhone from "../../assets/speakerPhone.svg";
import { Container, Sprite, Text } from "@pixi/react";
import Circle from "./Circle";
import { themeColors } from "../theme";
import { DisplayPosition } from "../Position";
import { TextStyle } from "pixi.js";

const speakerPhoneIconSize = 30;

interface Props {
  displayPosition: DisplayPosition;
  name: string;
}

const SpeakerPhone: React.FC<Props> = ({ displayPosition, name }) => {
  const [showName, setShowName] = useState(false);

  return (
    <Container x={displayPosition.left} y={displayPosition.top}>
      {showName && (
        <Text
          text={name}
          x={0}
          y={-speakerPhoneIconSize + 10}
          anchor={0.5}
          style={
            new TextStyle({
              fill: themeColors.textSecondary,
              fontSize: 14,
              fontWeight: "bold",
            })
          }
        />
      )}
      <Circle
        x={0}
        y={0}
        radius={200}
        lineWidth={2}
        color={themeColors.accentSecondary}
        fillColor={themeColors.accentSecondary}
        fillAlpha={0.1}
      />
      <Sprite
        image={speakerPhone}
        width={speakerPhoneIconSize}
        height={speakerPhoneIconSize}
        anchor={0.5}
        interactive
        onmouseover={() => setShowName(true)}
        onmouseout={() => setShowName(false)}
      />
    </Container>
  );
};

export default SpeakerPhone;
