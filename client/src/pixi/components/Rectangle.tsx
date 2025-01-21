import { Graphics } from "@pixi/react";
import { useCallback } from "react";
import * as PIXI from "pixi.js";

interface Props {
  lineWidth: number;
  color: number;
  width: number;
  height: number;
  fillColor?: number;
  fillAlpha?: number;
}

const Rectangle: React.FC<Props> = (props) => {
  const draw = useCallback(
    (g: PIXI.Graphics) => {
      g.clear();
      g.lineStyle(props.lineWidth, props.color);
      if (props.fillColor !== undefined) {
        g.beginFill(props.fillColor, props.fillAlpha ?? 1);
      }
      g.drawRect(
        props.lineWidth,
        props.lineWidth,
        props.width - 2 * props.lineWidth,
        props.height - 2 * props.lineWidth,
      );
      if (props.fillColor !== undefined) {
        g.endFill();
      }
    },
    [props],
  );

  return <Graphics draw={draw} />;
};

export default Rectangle;
