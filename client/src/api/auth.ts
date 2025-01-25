import useSWR from "swr";
import { AuthServiceClient } from "../schema2/auth.client";
import { TRANSPORT } from "./transport";
import useSWRMutation from "swr/mutation";

const authClient = new AuthServiceClient(TRANSPORT);

function fetcher() {
  return authClient.auth({}).response;
}
export const useAuth = () => {
  return useSWRMutation("grpc:auth", fetcher);
};
