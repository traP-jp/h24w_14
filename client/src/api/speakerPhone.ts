import useSWR from "swr";
import { SpeakerPhoneServiceClient } from "../schema/Speaker_phoneServiceClientPb";
import serverHostName from "./hostname";
import {
  GetSpeakerPhoneRequest,
  CreateSpeakerPhoneRequest,
  GetAvailableChannelsRequest,
  SearchChannelsRequest,
} from "../schema/speaker_phone_pb";
import useSWRMutation from "swr/mutation";
import { Coordinate } from "../schema/world_pb";
import { Position } from "../Position";

const speakerPhoneClient = new SpeakerPhoneServiceClient(serverHostName);

export const useSpeakerPhone = (speakerPhoneId: string) => {
  const req = new GetSpeakerPhoneRequest();
  req.setId(speakerPhoneId);
  const fetcher = () => speakerPhoneClient.getSpeakerPhone(req);
  return useSWR(`speakerPhone/${speakerPhoneId}`, fetcher);
};

export const useCreateSpeakerPhone = (position: Position, name: string) => {
  const req = new CreateSpeakerPhoneRequest();
  const coord = new Coordinate();
  coord.setX(position.x);
  coord.setY(position.y);
  req.setPosition(coord);
  req.setName(name);
  const fetcher = () => speakerPhoneClient.createSpeakerPhone(req);
  return useSWRMutation(`createSpeakerPhone`, fetcher);
};

export const useAvailableChannels = () => {
  const req = new GetAvailableChannelsRequest();
  const fetcher = () => speakerPhoneClient.getAvailableChannels(req);
  return useSWR(`availableChannels`, fetcher);
};

export const useSearchChannels = (name: string) => {
  const req = new SearchChannelsRequest();
  req.setName(name);
  const fetcher = () => speakerPhoneClient.searchChannels(req);
  return useSWR(`searchChannels`, fetcher);
};
