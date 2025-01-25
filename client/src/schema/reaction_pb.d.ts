import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as world_pb from './world_pb'; // proto import: "world.proto"


export class Reaction extends jspb.Message {
  getId(): string;
  setId(value: string): Reaction;

  getUserId(): string;
  setUserId(value: string): Reaction;

  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): Reaction;
  hasPosition(): boolean;
  clearPosition(): Reaction;

  getKind(): string;
  setKind(value: string): Reaction;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Reaction;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Reaction;

  getExpiresAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setExpiresAt(value?: google_protobuf_timestamp_pb.Timestamp): Reaction;
  hasExpiresAt(): boolean;
  clearExpiresAt(): Reaction;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Reaction.AsObject;
  static toObject(includeInstance: boolean, msg: Reaction): Reaction.AsObject;
  static serializeBinaryToWriter(message: Reaction, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Reaction;
  static deserializeBinaryFromReader(message: Reaction, reader: jspb.BinaryReader): Reaction;
}

export namespace Reaction {
  export type AsObject = {
    id: string,
    userId: string,
    position?: world_pb.Coordinate.AsObject,
    kind: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    expiresAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetReactionRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetReactionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetReactionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetReactionRequest): GetReactionRequest.AsObject;
  static serializeBinaryToWriter(message: GetReactionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetReactionRequest;
  static deserializeBinaryFromReader(message: GetReactionRequest, reader: jspb.BinaryReader): GetReactionRequest;
}

export namespace GetReactionRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetReactionResponse extends jspb.Message {
  getReaction(): Reaction | undefined;
  setReaction(value?: Reaction): GetReactionResponse;
  hasReaction(): boolean;
  clearReaction(): GetReactionResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetReactionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetReactionResponse): GetReactionResponse.AsObject;
  static serializeBinaryToWriter(message: GetReactionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetReactionResponse;
  static deserializeBinaryFromReader(message: GetReactionResponse, reader: jspb.BinaryReader): GetReactionResponse;
}

export namespace GetReactionResponse {
  export type AsObject = {
    reaction?: Reaction.AsObject,
  }
}

export class CreateReactionRequest extends jspb.Message {
  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): CreateReactionRequest;
  hasPosition(): boolean;
  clearPosition(): CreateReactionRequest;

  getKind(): string;
  setKind(value: string): CreateReactionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateReactionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateReactionRequest): CreateReactionRequest.AsObject;
  static serializeBinaryToWriter(message: CreateReactionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateReactionRequest;
  static deserializeBinaryFromReader(message: CreateReactionRequest, reader: jspb.BinaryReader): CreateReactionRequest;
}

export namespace CreateReactionRequest {
  export type AsObject = {
    position?: world_pb.Coordinate.AsObject,
    kind: string,
  }
}

export class CreateReactionResponse extends jspb.Message {
  getReaction(): Reaction | undefined;
  setReaction(value?: Reaction): CreateReactionResponse;
  hasReaction(): boolean;
  clearReaction(): CreateReactionResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateReactionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateReactionResponse): CreateReactionResponse.AsObject;
  static serializeBinaryToWriter(message: CreateReactionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateReactionResponse;
  static deserializeBinaryFromReader(message: CreateReactionResponse, reader: jspb.BinaryReader): CreateReactionResponse;
}

export namespace CreateReactionResponse {
  export type AsObject = {
    reaction?: Reaction.AsObject,
  }
}

