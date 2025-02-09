import useSWR from "swr";
import { ReactionServiceClient } from "../schema2/reaction.client";
import { GetReactionRequest, CreateReactionRequest } from "../schema2/reaction";
import useSWRMutation from "swr/mutation";
import { TRANSPORT } from "./transport";

const reactionClient = new ReactionServiceClient(TRANSPORT);

function getReactionFetcher([_, reactionId]: [unknown, string]) {
  const req: GetReactionRequest = { id: reactionId };
  return reactionClient.getReaction(req).response;
}
export const useReaction = (reactionId: string) => {
  return useSWR(["grpc:reaction", reactionId], getReactionFetcher);
};

function createReactionFetcher(
  _: unknown,
  { arg }: { arg: CreateReactionRequest },
) {
  return reactionClient.createReaction(arg).response;
}
export const useCreateReaction = () => {
  return useSWRMutation("grpc:createReaction", createReactionFetcher);
};
