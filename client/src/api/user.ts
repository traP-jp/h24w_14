import useSWR from "swr";
import { UserServiceClient } from "../schema/UserServiceClientPb";
import serverHostName from "./hostname";
import * as UserPb from "../schema/user_pb";

const userClient = new UserServiceClient(serverHostName);

export const useUser = (userId: string) => {
  const req = new UserPb.GetUserRequest();
  req.setId(userId);
  const fetcher = () => userClient.getUser(req);
  return useSWR(`user/${userId}`, fetcher);
};

export const useMe = () => {
  const req = new UserPb.GetMeRequest();
  const fetcher = () => userClient.getMe(req);
  return useSWR(`user/me`, fetcher);
};
