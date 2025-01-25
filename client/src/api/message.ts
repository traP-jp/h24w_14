import useSWR from "swr";
import { MessageServiceClient } from "../schema2/message.client";
import serverHostName from "./hostname";
import { CreateMessageRequest, GetMessageRequest } from "../schema2/message";
import useSWRMutation from "swr/mutation";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const messageClient = new MessageServiceClient(transport);

function getMessageFetcher([_, messageId]: [unknown, string]) {
  const req: GetMessageRequest = { id: messageId };
  return messageClient.getMessage(req).response;
}
export const useMessage = (messageId: string) => {
  return useSWR(["grpc:message", messageId], getMessageFetcher);
};

function createMessageFetcher(
  _: unknown,
  { arg }: { arg: CreateMessageRequest },
) {
  return messageClient.createMessage(arg).response;
}

export const useCreateMessage = () => {
  return useSWRMutation(`grpc:createMessage`, createMessageFetcher);
};
