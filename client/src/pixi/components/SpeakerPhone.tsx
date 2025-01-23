import React from "react";
import speakerPhone from "../../assets/speakerPhone.svg";
import { Container, Sprite } from "@pixi/react";
import Circle from "./Circle";
import { themeColors } from "../theme";

const speakerPhoneIconSize = 30;

const SpeakerPhone: React.FC = () => {
  return (
    <Container x={200} y={200}>
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
      />
    </Container>
  );
};

export default SpeakerPhone;
