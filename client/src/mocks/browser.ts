import { setupWorker } from "msw/browser";

const handlers = [].flat();

export const initMock = () => {
  if (process.env.NODE_ENV === "development") {
    const worker = setupWorker(...handlers);
    worker.start({
      onUnhandledRequest: "bypass",
    });
  }
};
