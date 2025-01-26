const host =
  URL.parse(import.meta.env.BASE_URL)?.host ?? "ping-point.trap.show";

const serverHostName = import.meta.env.DEV
  ? "http://localhost:8000"
  : `https://${host}`;

export const serverWSHostName = import.meta.env.DEV
  ? "ws://localhost:8000/ws"
  : `wss://${host}/ws`;

export default serverHostName;
