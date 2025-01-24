import { Graphics, Container } from "@pixi/react";
import React, { useEffect, useRef, useState } from "react";
import PIXI, { ColorSource } from "pixi.js";

interface Props {
  x: number;
  y: number;
  padding: number;
  lineWidth: number;
  radius: number;
  direction?: "up";
  color: ColorSource;
  fillColor?: ColorSource;
  fillAlpha?: number;
  children?: React.ReactNode;
}

const bubbleTriangleHeight = 5;
const bubbleTriangleWidth = 10;

const Bubble: React.FC<Props> = (props) => {
  const containerRef = useRef<PIXI.Container>(null);
  const [containerWidth, setContainerWidth] = useState(0);
  const [containerHeight, setContainerHeight] = useState(0);

  const draw = (g: PIXI.Graphics) => {
    g.clear();
    g.lineStyle(props.lineWidth, props.color);
    if (props.fillColor !== undefined) {
      g.beginFill(props.fillColor, props.fillAlpha ?? 1);
    }
    g.drawRoundedRect(0, 0, containerWidth, containerHeight, props.radius);
    g.moveTo(containerWidth / 2, containerHeight + bubbleTriangleHeight);
    g.lineTo(containerWidth / 2 - bubbleTriangleWidth / 2, containerHeight);
    g.lineTo(containerWidth / 2 + bubbleTriangleWidth / 2, containerHeight);
    g.lineTo(containerWidth / 2, containerHeight + bubbleTriangleHeight);
    if (props.fillColor !== undefined) {
      g.endFill();
    }
  };

  useEffect(() => {
    if (containerRef.current) {
      const containerRec = containerRef.current.getBounds();
      setContainerWidth(containerRec.width + props.padding * 2);
      setContainerHeight(containerRec.height + props.padding * 2);
    }
  }, [containerRef, props.padding]);

  return (
    <>
      <Graphics
        draw={draw}
        x={-containerWidth / 2 + props.x}
        y={-containerHeight + props.y - props.padding}
      />
      <Container
        x={-containerWidth / 2 + props.padding}
        y={-containerHeight / 2 - containerHeight}
        ref={containerRef}
      >
        {props.children}
      </Container>
    </>
  );
};

export default Bubble;
