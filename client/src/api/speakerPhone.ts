import useSWR from "swr";
import { SpeakerPhoneServiceClient } from "../schema2/speaker_phone.client";
import serverHostName from "./hostname";
import { GetSpeakerPhoneRequest, CreateSpeakerPhoneRequest, GetAvailableChannelsRequest, SearchChannelsRequest } from "../schema2/speaker_phone";
import useSWRMutation from "swr/mutation";
import { Position } from "../model/position";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

const transport = new GrpcWebFetchTransport({
  baseUrl: serverHostName,
});
const speakerPhoneClient = new SpeakerPhoneServiceClient(transport);

export const useSpeakerPhone = (speakerPhoneId: string) => {
  const req: GetSpeakerPhoneRequest = { id: speakerPhoneId };
  const fetcher = () => speakerPhoneClient.getSpeakerPhone(req).response;
  return useSWR(`speakerPhone/${speakerPhoneId}`, fetcher);
};

export const useCreateSpeakerPhone = (position: Position, name: string) => {
  const req: CreateSpeakerPhoneRequest = { position, name };
  const fetcher = () => speakerPhoneClient.createSpeakerPhone(req).response;
  return useSWRMutation(`createSpeakerPhone`, fetcher);
};

export const useAvailableChannels = () => {
  const req: GetAvailableChannelsRequest = {};
  const fetcher = () => speakerPhoneClient.getAvailableChannels(req).response;
  return useSWR(`availableChannels`, fetcher);
};

export const useSearchChannels = (name: string) => {
  const req: SearchChannelsRequest = { name };
  const fetcher = () => speakerPhoneClient.searchChannels(req).response;
  return useSWR(`searchChannels`, fetcher);
};
