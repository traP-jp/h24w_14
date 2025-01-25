import { useSetAtom } from "jotai";
import { Position } from "../model/position";
import * as ExplorePb from "../schema/explore_pb";
import * as WorldPb from "../schema/world_pb";
import { serverWSHostName } from "./hostname";
import { useEffect, useRef } from "react";
import fieldMessagesAtom from "../state/message";
import fieldReactionsAtom from "../state/reactions";
import { ReactionName } from "../model/reactions";
import fieldSpeakerPhonesAtom from "../state/speakerPhone";
import fieldExplorersAtom from "../state/explorer";
import { Message } from "../model/message";

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

  const dispatcher = ({ position, size }: ExplorerMessage) => {
    subscriberRef.current.dispatchEvent(
      new CustomEvent(explorerEvent, {
        detail: {
          position,
          size,
        },
      }),
    );
  };

  useEffect(() => {
    const ws = new WebSocket(serverWSHostName);
    subscriberRef.current.addEventListener(explorerEvent, (event: Event) => {
      const explorationField = new ExplorePb.ExplorationField();
      const coord = new WorldPb.Coordinate();
      const size = new WorldPb.Size();

      // @ts-expect-error event is CustomEvent
      const message = event.detail as ExplorerMessage;

      coord.setX(message.position.x);
      coord.setY(message.position.y);
      size.setWidth(message.size.width);
      size.setHeight(message.size.height);
      explorationField.setPosition(coord);
      explorationField.setSize(size);

      ws.send(JSON.stringify(explorationField.toObject()));
    });
    ws.onmessage = (event) => {
      if (event.type !== "text") {
        return;
      }
      const events = JSON.parse(
        event.data,
      ) as ExplorePb.ExplorationFieldEvents.AsObject;
      const now = new Date();
      setFieldMessages((messages) => {
        const newMessagesMap: Map<string, Message> = new Map();
        messages.forEach((message) => {
          if (message.expiresAt > now) {
            newMessagesMap.set(message.id, message);
          }
        });
        events.messagesList.forEach((message) => {
          newMessagesMap.set(message.id, {
            id: message.id,
            userId: message.userId,
            content: message.content,
            createdAt: new Date(message.createdAt),
            updatedAt: new Date(message.updatedAt),
            expiresAt: new Date(message.expiresAt),
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
          ...events.reactionsList.map((reaction) => {
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
              createdAt: new Date(reaction.createdAt),
              expiresAt: new Date(reaction.expiresAt),
            };
          }),
        ];
      });
      setFieldSpeakerPhones((speakerPhones) => {
        return [
          ...speakerPhones,
          ...events.speakerPhonesList.map((speakerPhone) => ({
            id: speakerPhone.id,
            position: {
              x: speakerPhone.position?.x ?? 0,
              y: speakerPhone.position?.y ?? 0,
            },
            receiveRange: speakerPhone.receiveRange,
            name: speakerPhone.name,
            createdAt: new Date(speakerPhone.createdAt),
            updatedAt: new Date(speakerPhone.updatedAt),
          })),
        ];
      });
      setFieldExplorers((explorers) => {
        const explorerActions = events.explorerActionsList;
        explorerActions.forEach((action) => {
          if (action.arrive) {
            const explorer = action.arrive.explorer;
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
          if (action.leave) {
            explorers.delete(action.leave.id);
          }
          if (action.move) {
            const explorer = action.move.explorer;
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
