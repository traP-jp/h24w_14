import useSWR from "swr";
import { UserServiceClient } from "../schema/UserServiceClientPb";
import serverHostName from "./hostname";
import { GetUserRequest } from "../schema/user_pb";

const userClient = new UserServiceClient(serverHostName);

export const useUser = (userId: string) => {
  const req = new GetUserRequest();
  req.setId(userId);
  const fetcher = () => userClient.getUser(req);
  return useSWR(`user/${userId}`, fetcher);
};
