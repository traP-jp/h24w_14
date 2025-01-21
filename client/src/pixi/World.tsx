import { Container, Sprite } from "@pixi/react";
import Rectangle from "./components/Rectangle";
import "@pixi/events";

interface Point {
  x: number;
  y: number;
}

interface WorldProps {
  userPosition: Point;
  userDisplayPosition: Point;
}

const World = (props: WorldProps) => {
  const { userPosition, userDisplayPosition } = props;

  return (
    <Container
      width={2000}
      height={2000}
      x={-userPosition.x + userDisplayPosition.x}
      y={-userPosition.y + userDisplayPosition.y}
      anchor={{ x: 0, y: 0 }}
      interactive={true}
    >
      <Rectangle
        lineWidth={2}
        color={0xffffff}
        width={2000}
        height={2000}
        fillColor={0xeeeeee}
        fillAlpha={1}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={0}
        y={0}
        width={100}
        height={100}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={0}
        y={1900}
        width={100}
        height={100}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={1900}
        y={0}
        width={100}
        height={100}
      />
      <Sprite
        image={"https://q.trap.jp/api/v3/public/icon/ikura-hamu"}
        x={1900}
        y={1900}
        width={100}
        height={100}
      />
    </Container>
  );
};

export default World;
