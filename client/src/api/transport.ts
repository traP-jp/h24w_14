import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import serverHostName from "./hostname";

export const TRANSPORT = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
  fetchInit: {
    credentials: "include",
  },
});
