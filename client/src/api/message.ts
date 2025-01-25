import useSWR from "swr";
import { MessageServiceClient } from "../schema/MessageServiceClientPb";
import serverHostName from "./hostname";
import * as MessagePb from "../schema/message_pb";
import useSWRMutation from "swr/mutation";
import * as WorldPb from "../schema/world_pb";
import { Position } from "../model/position";

const messageClient = new MessageServiceClient(serverHostName);

export const useMessage = (messageId: string) => {
  const req = new MessagePb.GetMessageRequest();
  req.setId(messageId);
  const fetcher = () => messageClient.getMessage(req);
  return useSWR(`message/${messageId}`, fetcher);
};

export const useCreateMessage = (content: string, position: Position) => {
  const req = new MessagePb.CreateMessageRequest();
  req.setContent(content);
  const coord = new WorldPb.Coordinate();
  coord.setX(position.x);
  coord.setY(position.y);
  req.setPosition(coord);
  const fetcher = () => messageClient.createMessage(req);
  return useSWRMutation(`createMessage`, fetcher);
};
