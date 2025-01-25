import useSWR from "swr";
import { WorldServiceClient } from "../schema/WorldServiceClientPb";
import serverHostName from "./hostname";
import * as WorldPb from "../schema/world_pb";

const worldClient = new WorldServiceClient(serverHostName);

export const useWorld = () => {
  const req = new WorldPb.GetWorldRequest();
  const fetcher = () => worldClient.getWorld(req);
  return useSWR("world", fetcher);
};
