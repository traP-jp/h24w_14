import useSWR from "swr";
import serverHostName from "./hostname";
import { AuthServiceClient } from "../schema2/auth.client";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const authClient = new AuthServiceClient(transport);

function fetcher() {
  return authClient.auth({}).response;
}
export const useAuth = () => {
  return useSWR("grpc:auth", fetcher);
};
