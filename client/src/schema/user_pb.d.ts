import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class User extends jspb.Message {
  getId(): string;
  setId(value: string): User;

  getName(): string;
  setName(value: string): User;

  getDisplayName(): string;
  setDisplayName(value: string): User;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): User;
  hasCreatedAt(): boolean;
  clearCreatedAt(): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
  export type AsObject = {
    id: string,
    name: string,
    displayName: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetMeRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetMeRequest): GetMeRequest.AsObject;
  static serializeBinaryToWriter(message: GetMeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMeRequest;
  static deserializeBinaryFromReader(message: GetMeRequest, reader: jspb.BinaryReader): GetMeRequest;
}

export namespace GetMeRequest {
  export type AsObject = {
  }
}

export class GetMeResponse extends jspb.Message {
  getUser(): User | undefined;
  setUser(value?: User): GetMeResponse;
  hasUser(): boolean;
  clearUser(): GetMeResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetMeResponse): GetMeResponse.AsObject;
  static serializeBinaryToWriter(message: GetMeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMeResponse;
  static deserializeBinaryFromReader(message: GetMeResponse, reader: jspb.BinaryReader): GetMeResponse;
}

export namespace GetMeResponse {
  export type AsObject = {
    user?: User.AsObject,
  }
}

export class GetUserRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetUserRequest): GetUserRequest.AsObject;
  static serializeBinaryToWriter(message: GetUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUserRequest;
  static deserializeBinaryFromReader(message: GetUserRequest, reader: jspb.BinaryReader): GetUserRequest;
}

export namespace GetUserRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetUserResponse extends jspb.Message {
  getUser(): User | undefined;
  setUser(value?: User): GetUserResponse;
  hasUser(): boolean;
  clearUser(): GetUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetUserResponse): GetUserResponse.AsObject;
  static serializeBinaryToWriter(message: GetUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUserResponse;
  static deserializeBinaryFromReader(message: GetUserResponse, reader: jspb.BinaryReader): GetUserResponse;
}

export namespace GetUserResponse {
  export type AsObject = {
    user?: User.AsObject,
  }
}

