import { Graphics } from "@pixi/react";
import { ColorSource } from "pixi.js";
import React from "react";

interface Props {
  x: number;
  y: number;
  lineWidth: number;
  color: ColorSource;
  radius: number;
  fillColor?: number;
  fillAlpha?: number;
}

const Circle: React.FC<Props> = (props) => {
  return (
    <Graphics
      draw={(g) => {
        g.clear();
        g.lineStyle({ width: props.lineWidth, color: props.color });
        if (props.fillColor) {
          g.beginFill(props.fillColor, props.fillAlpha);
        }
        g.drawCircle(0, 0, props.radius);
        g.endFill();
      }}
      x={props.x}
      y={props.y}
    />
  );
};

export default Circle;
