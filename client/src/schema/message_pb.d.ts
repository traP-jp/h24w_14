import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as world_pb from './world_pb'; // proto import: "world.proto"


export class Message extends jspb.Message {
  getId(): string;
  setId(value: string): Message;

  getUserId(): string;
  setUserId(value: string): Message;

  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): Message;
  hasPosition(): boolean;
  clearPosition(): Message;

  getContent(): string;
  setContent(value: string): Message;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Message;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Message;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Message;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): Message;

  getExpiresAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setExpiresAt(value?: google_protobuf_timestamp_pb.Timestamp): Message;
  hasExpiresAt(): boolean;
  clearExpiresAt(): Message;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Message.AsObject;
  static toObject(includeInstance: boolean, msg: Message): Message.AsObject;
  static serializeBinaryToWriter(message: Message, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Message;
  static deserializeBinaryFromReader(message: Message, reader: jspb.BinaryReader): Message;
}

export namespace Message {
  export type AsObject = {
    id: string,
    userId: string,
    position?: world_pb.Coordinate.AsObject,
    content: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    expiresAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetMessageRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetMessageRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMessageRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetMessageRequest): GetMessageRequest.AsObject;
  static serializeBinaryToWriter(message: GetMessageRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMessageRequest;
  static deserializeBinaryFromReader(message: GetMessageRequest, reader: jspb.BinaryReader): GetMessageRequest;
}

export namespace GetMessageRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetMessageResponse extends jspb.Message {
  getMessage(): Message | undefined;
  setMessage(value?: Message): GetMessageResponse;
  hasMessage(): boolean;
  clearMessage(): GetMessageResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMessageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetMessageResponse): GetMessageResponse.AsObject;
  static serializeBinaryToWriter(message: GetMessageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMessageResponse;
  static deserializeBinaryFromReader(message: GetMessageResponse, reader: jspb.BinaryReader): GetMessageResponse;
}

export namespace GetMessageResponse {
  export type AsObject = {
    message?: Message.AsObject,
  }
}

export class CreateMessageRequest extends jspb.Message {
  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): CreateMessageRequest;
  hasPosition(): boolean;
  clearPosition(): CreateMessageRequest;

  getContent(): string;
  setContent(value: string): CreateMessageRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateMessageRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateMessageRequest): CreateMessageRequest.AsObject;
  static serializeBinaryToWriter(message: CreateMessageRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateMessageRequest;
  static deserializeBinaryFromReader(message: CreateMessageRequest, reader: jspb.BinaryReader): CreateMessageRequest;
}

export namespace CreateMessageRequest {
  export type AsObject = {
    position?: world_pb.Coordinate.AsObject,
    content: string,
  }
}

export class CreateMessageResponse extends jspb.Message {
  getMessage(): Message | undefined;
  setMessage(value?: Message): CreateMessageResponse;
  hasMessage(): boolean;
  clearMessage(): CreateMessageResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateMessageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateMessageResponse): CreateMessageResponse.AsObject;
  static serializeBinaryToWriter(message: CreateMessageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateMessageResponse;
  static deserializeBinaryFromReader(message: CreateMessageResponse, reader: jspb.BinaryReader): CreateMessageResponse;
}

export namespace CreateMessageResponse {
  export type AsObject = {
    message?: Message.AsObject,
  }
}

