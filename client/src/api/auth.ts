import useSWR from "swr";
import { AuthServiceClient } from "../schema2/auth.client";
import { TRANSPORT } from "./transport";

const authClient = new AuthServiceClient(TRANSPORT);

function fetcher() {
  return authClient.auth({}).response;
}
export const useAuth = () => {
  return useSWR("grpc:auth", fetcher);
};
