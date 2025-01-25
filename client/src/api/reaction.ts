import useSWR from "swr";
import { ReactionServiceClient } from "../schema/ReactionServiceClientPb";
import serverHostName from "./hostname";
import * as ReactionPb from "../schema/reaction_pb";
import { Position } from "../model/position";
import { ReactionName } from "../model/reactions";
import useSWRMutation from "swr/mutation";
import * as WorldPb from "../schema/world_pb";

const reactionClient = new ReactionServiceClient(serverHostName);

export const useReaction = (reactionId: string) => {
  const req = new ReactionPb.GetReactionRequest();
  req.setId(reactionId);
  const fetcher = () => reactionClient.getReaction(req);
  return useSWR(`reaction/${reactionId}`, fetcher);
};

export const useCreateReaction = (
  position: Position,
  reaction: ReactionName,
) => {
  const req = new ReactionPb.CreateReactionRequest();
  const coordinate = new WorldPb.Coordinate();
  coordinate.setX(position.x);
  coordinate.setY(position.y);
  req.setPosition(coordinate);
  req.setKind(reaction);
  const fetcher = () => reactionClient.createReaction(req);
  return useSWRMutation(`createReaction`, fetcher);
};
