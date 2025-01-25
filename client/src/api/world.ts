import useSWR from "swr";
import { WorldServiceClient } from "../schema2/world.client";
import { TRANSPORT } from "./transport";
import useSWRImmutable from "swr/immutable";

const worldClient = new WorldServiceClient(TRANSPORT);

function fetcher() {
  return worldClient.getWorld({}).response;
}
export const useWorld = () => {
  return useSWRImmutable("grpc:world", fetcher);
};
