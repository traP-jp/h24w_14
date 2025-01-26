import { useAtomValue, useSetAtom } from "jotai";
import { Position } from "../model/position";
import { ExplorationField, ExplorationFieldEvents } from "../schema2/explore";
import { serverWSHostName } from "./hostname";
import { useCallback, useEffect, useRef, useState } from "react";
import fieldMessagesAtom from "../state/message";
import fieldReactionsAtom from "../state/reactions";
import { ReactionName } from "../model/reactions";
import fieldSpeakerPhonesAtom from "../state/speakerPhone";
import fieldExplorersAtom from "../state/explorer";
import { Message } from "../model/message";
import type { ExplorerAction } from "../model/ExplorerAction";
import userPositionAtom from "../state/userPosition";
import fieldSizeAtom from "../state/field";

type ExplorerMessage = {
  position: Position;
  size: { width: number; height: number };
};

export type ExplorerMessageDispatcher = (message: ExplorerMessage) => void;

const explorerEvent = "explorer";

const useExplorerDispatcher = () => {
  const subscriber = new EventTarget();
  const subscriberRef = useRef<EventTarget>(subscriber);
  const userPosition = useAtomValue(userPositionAtom);
  const fieldSize = useAtomValue(fieldSizeAtom);
  const setFieldMessages = useSetAtom(fieldMessagesAtom);
  const setFieldReactions = useSetAtom(fieldReactionsAtom);
  const setFieldSpeakerPhones = useSetAtom(fieldSpeakerPhonesAtom);
  const setFieldExplorers = useSetAtom(fieldExplorersAtom);

  const [initialPosition, setInitialPosition] = useState<Position | null>(null);
  const [initialSize, setInitialSize] = useState<{
    width: number;
    height: number;
  } | null>(null);
  if (initialPosition === null && userPosition !== null) {
    setInitialPosition(userPosition);
  }
  if (initialSize === null && fieldSize !== null) {
    setInitialSize(fieldSize);
  }

  const dispatcher: ExplorerMessageDispatcher = useCallback((mes) => {
    if (!mes) return;
    subscriberRef.current.dispatchEvent(
      new CustomEvent(explorerEvent, {
        detail: {
          position: {
            x: Math.round(mes.position.x),
            y: Math.round(mes.position.y),
          },
          size: {
            width: Math.round(mes.size.width),
            height: Math.round(mes.size.height),
          },
        },
      }),
    );
  }, []);

  useEffect(() => {
    const ws = new WebSocket(serverWSHostName);
    const currentSubscriber = subscriberRef.current;
    const subscriverHandler = (event: Event) => {
      // @ts-expect-error event is CustomEvent
      const message = event.detail as ExplorerMessage;

      const explorationField: ExplorationField = {
        position: {
          x: message.position.x,
          y: message.position.y,
        },
        size: {
          ...message.size,
        },
      };

      ws.send(JSON.stringify(explorationField));
    };
    currentSubscriber.addEventListener(explorerEvent, subscriverHandler);

    ws.addEventListener("open", () => {
      if (initialPosition === null || initialSize === null) return;
      dispatcher({
        position: initialPosition,
        size: {
          width: initialSize.width,
          height: initialSize.height,
        },
      });
    });
    ws.onmessage = (event) => {
      if (event.type !== "message") {
        return;
      }
      const events = JSON.parse(event.data) as ExplorationFieldEvents;
      const now = new Date();
      setFieldMessages((messages) => {
        const newMessagesMap: Map<string, Message> = new Map();
        messages.forEach((message) => {
          // TODO: expireAt の判定を復活させる
          if (message.expiresAt > now) {
            newMessagesMap.set(message.id, message);
          }
        });
        events.messages.forEach((message) => {
          newMessagesMap.set(message.id, {
            id: message.id,
            userId: message.userId,
            content: message.content,
            createdAt: new Date(message.createdAt as unknown as string),
            updatedAt: new Date(message.updatedAt as unknown as string),
            expiresAt: new Date(message.expiresAt as unknown as string),
            position: {
              x: message.position?.x ?? 0,
              y: message.position?.y ?? 0,
            },
          });
        });
        return newMessagesMap;
      });
      setFieldReactions((reactions) => {
        return [
          ...reactions.filter((reaction) => reaction.expiresAt > now),
          ...events.reactions.map((reaction) => {
            const kind = reaction.kind as ReactionName;
            return {
              id: reaction.id,
              userId: reaction.userId,
              messageId: reaction.id,
              position: {
                x: reaction.position?.x ?? 0,
                y: reaction.position?.y ?? 0,
              },
              kind: kind,
              createdAt: new Date(reaction.createdAt as unknown as string),
              expiresAt: new Date(reaction.expiresAt as unknown as string),
            };
          }),
        ];
      });
      setFieldSpeakerPhones((speakerPhones) => {
        return [
          ...speakerPhones,
          ...events.speakerPhones.map((speakerPhone) => ({
            id: speakerPhone.id,
            position: {
              x: speakerPhone.position?.x ?? 0,
              y: speakerPhone.position?.y ?? 0,
            },
            receiveRange: speakerPhone.receiveRange,
            name: speakerPhone.name,
            createdAt: new Date(speakerPhone.createdAt as unknown as string),
            updatedAt: new Date(speakerPhone.updatedAt as unknown as string),
          })),
        ];
      });

      setFieldExplorers((explorers) => {
        // NOTE: バックエンドは proto に従ってない
        const explorerActions =
          events.explorerActions as unknown as ExplorerAction[];
        const newExplorers = new Map(explorers);
        explorerActions.forEach((action) => {
          switch (action.type) {
            case "arrive": {
              const explorer = action;
              newExplorers.set(explorer.id ?? "", {
                id: explorer.id ?? "",
                position: {
                  x: explorer.position?.x ?? 0,
                  y: explorer.position?.y ?? 0,
                },
                userId: explorer.inner.id ?? "",
              });
              break;
            }
            case "leave": {
              newExplorers.delete(action.id);
              break;
            }
            case "move": {
              const explorer = action;
              const prevExplorer = newExplorers.get(explorer.id ?? "");
              if (!prevExplorer) return;
              newExplorers.set(explorer.id ?? "", {
                id: explorer.id ?? "",
                position: {
                  x: explorer.position?.x ?? 0,
                  y: explorer.position?.y ?? 0,
                },
                userId: explorer.inner.id ?? "",
                previousPosition: prevExplorer.position,
              });
              break;
            }

            default:
              break;
          }
          if (action.type === "arrive") {
            const explorer = action;
            newExplorers.set(explorer.id ?? "", {
              id: explorer.id ?? "",
              position: {
                x: explorer.position?.x ?? 0,
                y: explorer.position?.y ?? 0,
              },
              userId: explorer.inner.id ?? "",
            });
          }
        });
        return newExplorers;
      });
    };
    return () => {
      ws.close();
      currentSubscriber.removeEventListener(explorerEvent, subscriverHandler);
    };
  }, [
    setFieldMessages,
    setFieldReactions,
    setFieldSpeakerPhones,
    setFieldExplorers,
    dispatcher,
    initialPosition,
    initialSize,
  ]);

  return dispatcher;
};

export default useExplorerDispatcher;
