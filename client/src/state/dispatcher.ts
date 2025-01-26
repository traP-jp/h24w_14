import { atom, useAtomValue } from "jotai";
import { ExplorerMessageDispatcher } from "../api/explorer";
import { useMemo } from "react";
import { throttle } from "throttle-debounce";

const dispatcherAtom = atom<ExplorerMessageDispatcher | undefined>(undefined);
export function useThrottledDispatcher() {
  const dispatcher = useAtomValue(dispatcherAtom);

  const throttled = useMemo(
    () => (dispatcher !== undefined ? throttle(100, dispatcher) : undefined),
    [dispatcher],
  );
  return throttled;
}

export default dispatcherAtom;
