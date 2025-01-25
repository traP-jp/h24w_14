import useSWR from "swr";
import { AuthServiceClient } from "../schema/AuthServiceClientPb";
import serverHostName from "./hostname";
import * as AuthPb from "../schema/auth_pb";

const authClient = new AuthServiceClient(serverHostName);

export const useAuth = () => {
  const req = new AuthPb.AuthRequest();
  const fetcher = () => authClient.auth(req);
  return useSWR(`auth`, fetcher);
};
