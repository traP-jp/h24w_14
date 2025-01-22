// Code generated by protoc-gen-ts_proto. DO NOT EDIT.
// versions:
//   protoc-gen-ts_proto  v2.6.1
//   protoc               v3.12.4
// source: auth.proto

/* eslint-disable */
import { BinaryReader, BinaryWriter } from "@bufbuild/protobuf/wire";

export const protobufPackage = "auth";

export interface AuthRequest {
}

export interface AuthResponse {
  /** リダイレクト先 */
  location: string;
}

function createBaseAuthRequest(): AuthRequest {
  return {};
}

export const AuthRequest: MessageFns<AuthRequest> = {
  encode(_: AuthRequest, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): AuthRequest {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAuthRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): AuthRequest {
    return {};
  },

  toJSON(_: AuthRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<AuthRequest>, I>>(base?: I): AuthRequest {
    return AuthRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AuthRequest>, I>>(_: I): AuthRequest {
    const message = createBaseAuthRequest();
    return message;
  },
};

function createBaseAuthResponse(): AuthResponse {
  return { location: "" };
}

export const AuthResponse: MessageFns<AuthResponse> = {
  encode(message: AuthResponse, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.location !== "") {
      writer.uint32(10).string(message.location);
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): AuthResponse {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAuthResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 10) {
            break;
          }

          message.location = reader.string();
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AuthResponse {
    return { location: isSet(object.location) ? globalThis.String(object.location) : "" };
  },

  toJSON(message: AuthResponse): unknown {
    const obj: any = {};
    if (message.location !== "") {
      obj.location = message.location;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AuthResponse>, I>>(base?: I): AuthResponse {
    return AuthResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AuthResponse>, I>>(object: I): AuthResponse {
    const message = createBaseAuthResponse();
    message.location = object.location ?? "";
    return message;
  },
};

export interface AuthService {
  /**
   * OAuth認証
   * レスポンスのlocationにリダイレクトすることでOAuth認証を行う
   */
  Auth(request: AuthRequest): Promise<AuthResponse>;
}

export const AuthServiceServiceName = "auth.AuthService";
export class AuthServiceClientImpl implements AuthService {
  private readonly rpc: Rpc;
  private readonly service: string;
  constructor(rpc: Rpc, opts?: { service?: string }) {
    this.service = opts?.service || AuthServiceServiceName;
    this.rpc = rpc;
    this.Auth = this.Auth.bind(this);
  }
  Auth(request: AuthRequest): Promise<AuthResponse> {
    const data = AuthRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "Auth", data);
    return promise.then((data) => AuthResponse.decode(new BinaryReader(data)));
  }
}

interface Rpc {
  request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

export interface MessageFns<T> {
  encode(message: T, writer?: BinaryWriter): BinaryWriter;
  decode(input: BinaryReader | Uint8Array, length?: number): T;
  fromJSON(object: any): T;
  toJSON(message: T): unknown;
  create<I extends Exact<DeepPartial<T>, I>>(base?: I): T;
  fromPartial<I extends Exact<DeepPartial<T>, I>>(object: I): T;
}
