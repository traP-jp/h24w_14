import useSWR from "swr";
import { MessageServiceClient } from "../schema2/message.client";
import serverHostName from "./hostname";
import { CreateMessageRequest, GetMessageRequest } from "../schema2/message";
import useSWRMutation from "swr/mutation";
import { Position } from "../model/position";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const messageClient = new MessageServiceClient(transport);

export const useMessage = (messageId: string) => {
  const req: GetMessageRequest = { id: messageId };
  const fetcher = () => messageClient.getMessage(req).response;
  return useSWR(`message/${messageId}`, fetcher);
};

export const useCreateMessage = (content: string, position: Position) => {
  const req: CreateMessageRequest = { content, position };
  const fetcher = () => messageClient.createMessage(req).response;
  return useSWRMutation(`createMessage`, fetcher);
};
