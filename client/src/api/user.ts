import { UserServiceClient } from "../schema2/user.client";
import { TRANSPORT } from "./transport";
import useSWRImmutable from "swr/immutable";

const userClient = new UserServiceClient(TRANSPORT);

function getUserFetcher([_, id]: [unknown, string]) {
  const req = {
    id: id,
  };
  return userClient.getUser(req).response;
}
export const useUser = (id: string) => {
  return useSWRImmutable(
    id !== "" ? ["grpc:user", id] : undefined,
    getUserFetcher,
  );
};

function getMeFetcher() {
  return userClient.getMe({}).response;
}
export const useMe = () => {
  return useSWRImmutable("grpc:user/me", getMeFetcher);
};
