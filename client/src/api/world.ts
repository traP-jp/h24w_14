import useSWR from "swr";
import { WorldServiceClient } from "../schema2/world.client";
import serverHostName from "./hostname";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const worldClient = new WorldServiceClient(transport);

export const useWorld = () => {
  const req = {};
  const fetcher = () => worldClient.getWorld(req).response;
  return useSWR("world", fetcher);
};
