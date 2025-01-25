import useSWR from "swr";
import { WorldServiceClient } from "../schema2/world.client";
import { TRANSPORT } from "./transport";

const worldClient = new WorldServiceClient(TRANSPORT);

function fetcher() {
  return worldClient.getWorld({}).response;
}
export const useWorld = () => {
  return useSWR("grpc:world", fetcher);
};
