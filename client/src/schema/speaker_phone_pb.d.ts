import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as world_pb from './world_pb'; // proto import: "world.proto"


export class SpeakerPhone extends jspb.Message {
  getId(): string;
  setId(value: string): SpeakerPhone;

  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): SpeakerPhone;
  hasPosition(): boolean;
  clearPosition(): SpeakerPhone;

  getReceiveRange(): number;
  setReceiveRange(value: number): SpeakerPhone;

  getName(): string;
  setName(value: string): SpeakerPhone;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SpeakerPhone;
  hasCreatedAt(): boolean;
  clearCreatedAt(): SpeakerPhone;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SpeakerPhone;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): SpeakerPhone;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SpeakerPhone.AsObject;
  static toObject(includeInstance: boolean, msg: SpeakerPhone): SpeakerPhone.AsObject;
  static serializeBinaryToWriter(message: SpeakerPhone, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SpeakerPhone;
  static deserializeBinaryFromReader(message: SpeakerPhone, reader: jspb.BinaryReader): SpeakerPhone;
}

export namespace SpeakerPhone {
  export type AsObject = {
    id: string,
    position?: world_pb.Coordinate.AsObject,
    receiveRange: number,
    name: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetSpeakerPhoneRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetSpeakerPhoneRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSpeakerPhoneRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSpeakerPhoneRequest): GetSpeakerPhoneRequest.AsObject;
  static serializeBinaryToWriter(message: GetSpeakerPhoneRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSpeakerPhoneRequest;
  static deserializeBinaryFromReader(message: GetSpeakerPhoneRequest, reader: jspb.BinaryReader): GetSpeakerPhoneRequest;
}

export namespace GetSpeakerPhoneRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetSpeakerPhoneResponse extends jspb.Message {
  getSpeakerPhone(): SpeakerPhone | undefined;
  setSpeakerPhone(value?: SpeakerPhone): GetSpeakerPhoneResponse;
  hasSpeakerPhone(): boolean;
  clearSpeakerPhone(): GetSpeakerPhoneResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSpeakerPhoneResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSpeakerPhoneResponse): GetSpeakerPhoneResponse.AsObject;
  static serializeBinaryToWriter(message: GetSpeakerPhoneResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSpeakerPhoneResponse;
  static deserializeBinaryFromReader(message: GetSpeakerPhoneResponse, reader: jspb.BinaryReader): GetSpeakerPhoneResponse;
}

export namespace GetSpeakerPhoneResponse {
  export type AsObject = {
    speakerPhone?: SpeakerPhone.AsObject,
  }
}

export class CreateSpeakerPhoneRequest extends jspb.Message {
  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): CreateSpeakerPhoneRequest;
  hasPosition(): boolean;
  clearPosition(): CreateSpeakerPhoneRequest;

  getName(): string;
  setName(value: string): CreateSpeakerPhoneRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSpeakerPhoneRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSpeakerPhoneRequest): CreateSpeakerPhoneRequest.AsObject;
  static serializeBinaryToWriter(message: CreateSpeakerPhoneRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSpeakerPhoneRequest;
  static deserializeBinaryFromReader(message: CreateSpeakerPhoneRequest, reader: jspb.BinaryReader): CreateSpeakerPhoneRequest;
}

export namespace CreateSpeakerPhoneRequest {
  export type AsObject = {
    position?: world_pb.Coordinate.AsObject,
    name: string,
  }
}

export class CreateSpeakerPhoneResponse extends jspb.Message {
  getSpeakerPhone(): SpeakerPhone | undefined;
  setSpeakerPhone(value?: SpeakerPhone): CreateSpeakerPhoneResponse;
  hasSpeakerPhone(): boolean;
  clearSpeakerPhone(): CreateSpeakerPhoneResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSpeakerPhoneResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSpeakerPhoneResponse): CreateSpeakerPhoneResponse.AsObject;
  static serializeBinaryToWriter(message: CreateSpeakerPhoneResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSpeakerPhoneResponse;
  static deserializeBinaryFromReader(message: CreateSpeakerPhoneResponse, reader: jspb.BinaryReader): CreateSpeakerPhoneResponse;
}

export namespace CreateSpeakerPhoneResponse {
  export type AsObject = {
    speakerPhone?: SpeakerPhone.AsObject,
  }
}

export class GetAvailableChannelsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAvailableChannelsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAvailableChannelsRequest): GetAvailableChannelsRequest.AsObject;
  static serializeBinaryToWriter(message: GetAvailableChannelsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAvailableChannelsRequest;
  static deserializeBinaryFromReader(message: GetAvailableChannelsRequest, reader: jspb.BinaryReader): GetAvailableChannelsRequest;
}

export namespace GetAvailableChannelsRequest {
  export type AsObject = {
  }
}

export class GetAvailableChannelsResponse extends jspb.Message {
  getChannelsList(): Array<string>;
  setChannelsList(value: Array<string>): GetAvailableChannelsResponse;
  clearChannelsList(): GetAvailableChannelsResponse;
  addChannels(value: string, index?: number): GetAvailableChannelsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAvailableChannelsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAvailableChannelsResponse): GetAvailableChannelsResponse.AsObject;
  static serializeBinaryToWriter(message: GetAvailableChannelsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAvailableChannelsResponse;
  static deserializeBinaryFromReader(message: GetAvailableChannelsResponse, reader: jspb.BinaryReader): GetAvailableChannelsResponse;
}

export namespace GetAvailableChannelsResponse {
  export type AsObject = {
    channelsList: Array<string>,
  }
}

export class SearchChannelsRequest extends jspb.Message {
  getName(): string;
  setName(value: string): SearchChannelsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SearchChannelsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SearchChannelsRequest): SearchChannelsRequest.AsObject;
  static serializeBinaryToWriter(message: SearchChannelsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SearchChannelsRequest;
  static deserializeBinaryFromReader(message: SearchChannelsRequest, reader: jspb.BinaryReader): SearchChannelsRequest;
}

export namespace SearchChannelsRequest {
  export type AsObject = {
    name: string,
  }
}

export class SearchChannelsResponse extends jspb.Message {
  getHitsList(): Array<string>;
  setHitsList(value: Array<string>): SearchChannelsResponse;
  clearHitsList(): SearchChannelsResponse;
  addHits(value: string, index?: number): SearchChannelsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SearchChannelsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SearchChannelsResponse): SearchChannelsResponse.AsObject;
  static serializeBinaryToWriter(message: SearchChannelsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SearchChannelsResponse;
  static deserializeBinaryFromReader(message: SearchChannelsResponse, reader: jspb.BinaryReader): SearchChannelsResponse;
}

export namespace SearchChannelsResponse {
  export type AsObject = {
    hitsList: Array<string>,
  }
}

