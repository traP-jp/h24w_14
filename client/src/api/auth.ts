import useSWR from "swr";
import { AuthServiceClient } from "../schema/AuthServiceClientPb";
import serverHostName from "./hostname";
import { AuthRequest } from "../schema/auth_pb";

const authClient = new AuthServiceClient(serverHostName);

export const useAuth = () => {
  const req = new AuthRequest();
  const fetcher = () => authClient.auth(req);
  return useSWR(`auth`, fetcher);
};
