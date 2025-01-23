import { Graphics } from "@pixi/react";
import React from "react";
import PIXI, { ColorSource } from "pixi.js";

interface Props {
  x: number;
  y: number;
  width: number;
  height: number;
  lineWidth: number;
  radius: number;
  direction?: "up";
  color: ColorSource;
  fillColor?: ColorSource;
  fillAlpha?: number;
}

const Bubble: React.FC<Props> = (props) => {
  const draw = (g: PIXI.Graphics) => {
    g.clear();
    g.lineStyle(props.lineWidth, props.color);
    if (props.fillColor !== undefined) {
      g.beginFill(props.fillColor, props.fillAlpha ?? 1);
    }
    g.drawRoundedRect(0, 0, props.width, props.height, props.radius);
    g.moveTo(props.width / 2, props.height + 5);
    g.lineTo(props.width / 2 - 5, props.height);
    g.lineTo(props.width / 2 + 5, props.height);
    g.lineTo(props.width / 2, props.height + 5);
    if (props.fillColor !== undefined) {
      g.endFill();
    }
  };
  return (
    <Graphics
      draw={draw}
      x={-props.width / 2 + props.x}
      y={-props.height + props.y}
    />
  );
};

export default Bubble;
