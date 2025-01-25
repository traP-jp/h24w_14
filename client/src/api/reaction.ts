import useSWR from "swr";
import { ReactionServiceClient } from "../schema2/reaction.client";
import serverHostName from "./hostname";
import { GetReactionRequest, CreateReactionRequest } from "../schema2/reaction";
import useSWRMutation from "swr/mutation";
import { Position } from "../model/position";
import { ReactionName } from "../model/reactions";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const reactionClient = new ReactionServiceClient(transport);

export const useReaction = (reactionId: string) => {
  const req: GetReactionRequest = { id: reactionId };
  const fetcher = () => reactionClient.getReaction(req).response;
  return useSWR(`reaction/${reactionId}`, fetcher);
};

export const useCreateReaction = (
  position: Position,
  reaction: ReactionName,
) => {
  const req: CreateReactionRequest = {
    position: { x: position.x, y: position.y },
    kind: reaction,
  };
  const fetcher = () => reactionClient.createReaction(req).response;
  return useSWRMutation(`createReaction`, fetcher);
};
