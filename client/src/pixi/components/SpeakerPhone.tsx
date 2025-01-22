import React from "react";
import speakerPhone from "../../assets/speakerPhone.svg";
import { Sprite } from "@pixi/react";

const speakerPhoneIconSize = 30;

const SpeakerPhone: React.FC = () => {
  return (
    <Sprite
      image={speakerPhone}
      x={200}
      y={200}
      width={speakerPhoneIconSize}
      height={speakerPhoneIconSize}
    />
  );
};

export default SpeakerPhone;
