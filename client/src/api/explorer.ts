import { useSetAtom } from "jotai";
import { Position } from "../model/position";
import { ExplorationField, ExplorationFieldEvents } from "../schema2/explore";
import { serverWSHostName } from "./hostname";
import { useEffect, useRef } from "react";
import fieldMessagesAtom from "../state/message";
import fieldReactionsAtom from "../state/reactions";
import { ReactionName } from "../model/reactions";
import fieldSpeakerPhonesAtom from "../state/speakerPhone";
import fieldExplorersAtom from "../state/explorer";
import { Message } from "../model/message";
import { Timestamp } from "../schema2/google/protobuf/timestamp";

type ExplorerMessage = {
  position: Position;
  size: { width: number; height: number };
};

export type ExplorerMessageDispatcher = (message: ExplorerMessage) => void;

const explorerEvent = "explorer";

const useExplorerDispatcher = () => {
  const subscriber = new EventTarget();
  const subscriberRef = useRef<EventTarget>(subscriber);
  const setFieldMessages = useSetAtom(fieldMessagesAtom);
  const setFieldReactions = useSetAtom(fieldReactionsAtom);
  const setFieldSpeakerPhones = useSetAtom(fieldSpeakerPhonesAtom);
  const setFieldExplorers = useSetAtom(fieldExplorersAtom);

  const dispatcher: ExplorerMessageDispatcher = (mes) => {
    if (!mes) return;
    subscriberRef.current.dispatchEvent(
      new CustomEvent(explorerEvent, {
        detail: {
          position: mes.position,
          size: mes.size,
        },
      }),
    );
  };

  useEffect(() => {
    const ws = new WebSocket(serverWSHostName);
    subscriberRef.current.addEventListener(explorerEvent, (event: Event) => {
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
    });
    ws.onmessage = (event) => {
      if (event.type !== "text") {
        return;
      }
      const events = JSON.parse(event.data) as ExplorationFieldEvents;
      const now = new Date();
      setFieldMessages((messages) => {
        const newMessagesMap: Map<string, Message> = new Map();
        messages.forEach((message) => {
          if (message.expiresAt > now) {
            newMessagesMap.set(message.id, message);
          }
        });
        events.messages.forEach((message) => {
          newMessagesMap.set(message.id, {
            id: message.id,
            userId: message.userId,
            content: message.content,
            createdAt: message.createdAt
              ? Timestamp.toDate(message.createdAt)
              : new Date(),
            updatedAt: message.updatedAt
              ? Timestamp.toDate(message.updatedAt)
              : new Date(),
            expiresAt: message.expiresAt
              ? Timestamp.toDate(message.expiresAt)
              : new Date(),
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
              createdAt: reaction.createdAt
                ? Timestamp.toDate(reaction.createdAt)
                : new Date(),
              expiresAt: reaction.expiresAt
                ? Timestamp.toDate(reaction.expiresAt)
                : new Date(),
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
            createdAt: speakerPhone.createdAt
              ? Timestamp.toDate(speakerPhone.createdAt)
              : new Date(),
            updatedAt: speakerPhone.updatedAt
              ? Timestamp.toDate(speakerPhone.updatedAt)
              : new Date(),
          })),
        ];
      });
      setFieldExplorers((explorers) => {
        const explorerActions = events.explorerActions;
        explorerActions.forEach((action) => {
          switch (action.action.oneofKind) {
            case "arrive": {
              const explorer = action.action.arrive.explorer;
              if (!explorer) return;
              explorers.set(explorer.id ?? "", {
                id: explorer.id ?? "",
                position: {
                  x: explorer.position?.x ?? 0,
                  y: explorer.position?.y ?? 0,
                },
                userId: explorer.userId ?? "",
              });
              break;
            }
            case "leave": {
              explorers.delete(action.action.leave.id);
              break;
            }
            case "move": {
              const explorer = action.action.move.explorer;
              if (!explorer) return;
              const prevExplorer = explorers.get(explorer.id ?? "");
              if (!prevExplorer) return;
              explorers.set(explorer.id ?? "", {
                id: explorer.id ?? "",
                position: {
                  x: explorer.position?.x ?? 0,
                  y: explorer.position?.y ?? 0,
                },
                userId: explorer.userId ?? "",
                previousPosition: prevExplorer.position,
              });
              break;
            }

            default:
              break;
          }
          if (action.action.oneofKind === "arrive") {
            const explorer = action.action.arrive.explorer;
            if (!explorer) return;
            explorers.set(explorer.id ?? "", {
              id: explorer.id ?? "",
              position: {
                x: explorer.position?.x ?? 0,
                y: explorer.position?.y ?? 0,
              },
              userId: explorer.userId ?? "",
            });
          }
        });
        return explorers;
      });
    };
    return () => {
      ws.close();
    };
  }, [
    setFieldMessages,
    setFieldReactions,
    setFieldSpeakerPhones,
    setFieldExplorers,
  ]);

  return dispatcher;
};

export default useExplorerDispatcher;
