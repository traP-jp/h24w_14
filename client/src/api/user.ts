import useSWR from "swr";
import serverHostName from "./hostname";
import { UserServiceClient } from "../schema2/user.client";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const userClient = new UserServiceClient(transport);

function getUserFetcher([_, id]: [unknown, string]) {
  const req = {
    id: id,
  };
  return userClient.getUser(req).response;
}
export const useUser = (id: string) => {
  return useSWR(["grpc:user", id], getUserFetcher);
};

function getMeFetcher() {
  return userClient.getMe({}).response;
}
export const useMe = () => {
  return useSWR("grpc:user/me", getMeFetcher);
};
