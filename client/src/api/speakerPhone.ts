import useSWR from "swr";
import { SpeakerPhoneServiceClient } from "../schema2/speaker_phone.client";
import serverHostName from "./hostname";
import {
  GetSpeakerPhoneRequest,
  CreateSpeakerPhoneRequest,
  SearchChannelsRequest,
} from "../schema2/speaker_phone";
import useSWRMutation from "swr/mutation";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const speakerPhoneClient = new SpeakerPhoneServiceClient(transport);

function getSpeakerPhoneFetcher([_, speakerPhoneId]: [unknown, string]) {
  const req: GetSpeakerPhoneRequest = { id: speakerPhoneId };
  return speakerPhoneClient.getSpeakerPhone(req).response;
}
export const useSpeakerPhone = (speakerPhoneId: string) => {
  return useSWR(["grpc:speakerPhone", speakerPhoneId], getSpeakerPhoneFetcher);
};

function createSpeakerPhoneFetcher(
  _: unknown,
  { arg }: { arg: CreateSpeakerPhoneRequest },
) {
  return speakerPhoneClient.createSpeakerPhone(arg).response;
}
export const useCreateSpeakerPhone = () => {
  return useSWRMutation("grpc:createSpeakerPhone", createSpeakerPhoneFetcher);
};

function getAvailableChannelsFetcher() {
  return speakerPhoneClient.getAvailableChannels({}).response;
}
export const useAvailableChannels = () => {
  return useSWR("grpc:availableChannels", getAvailableChannelsFetcher);
};

function searchChannelsFetcher([_, name]: [unknown, string]) {
  const req: SearchChannelsRequest = { name };
  return speakerPhoneClient.searchChannels(req).response;
}
export const useSearchChannels = (name: string) => {
  return useSWR(["grpc:searchChannels", name], searchChannelsFetcher);
};
