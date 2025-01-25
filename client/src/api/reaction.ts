import useSWR from "swr";
import { ReactionServiceClient } from "../schema/ReactionServiceClientPb";
import serverHostName from "./hostname";
import {
  CreateReactionRequest,
  GetReactionRequest,
} from "../schema/reaction_pb";
import { Position } from "../Position";
import { ReactionName } from "../reactions";
import useSWRMutation from "swr/mutation";
import { Coordinate } from "../schema/world_pb";

const reactionClient = new ReactionServiceClient(serverHostName);

export const useReaction = (reactionId: string) => {
  const req = new GetReactionRequest();
  req.setId(reactionId);
  const fetcher = () => reactionClient.getReaction(req);
  return useSWR(`reaction/${reactionId}`, fetcher);
};

export const useCreateReaction = (
  position: Position,
  reaction: ReactionName,
) => {
  const req = new CreateReactionRequest();
  const coordinate = new Coordinate();
  coordinate.setX(position.x);
  coordinate.setY(position.y);
  req.setPosition(coordinate);
  req.setKind(reaction);
  const fetcher = () => reactionClient.createReaction(req);
  return useSWRMutation(`createReaction`, fetcher);
};
