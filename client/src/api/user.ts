import useSWR from "swr";
import serverHostName from "./hostname";
import { UserServiceClient } from "../schema2/user.client";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const userClient = new UserServiceClient(transport);

export const useUser = (id: string) => {
  const req = {
    id: id,
  };
  const res = userClient.getUser(req);
  return useSWR(`user/${id}`, () => res.response);
};

export const useMe = () => {
  const req = {};
  const res = userClient.getMe(req);
  return useSWR(`user/me`, () => res.response);
};
