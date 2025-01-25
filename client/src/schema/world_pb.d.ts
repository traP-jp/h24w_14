import * as jspb from 'google-protobuf'



export class Size extends jspb.Message {
  getWidth(): number;
  setWidth(value: number): Size;

  getHeight(): number;
  setHeight(value: number): Size;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Size.AsObject;
  static toObject(includeInstance: boolean, msg: Size): Size.AsObject;
  static serializeBinaryToWriter(message: Size, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Size;
  static deserializeBinaryFromReader(message: Size, reader: jspb.BinaryReader): Size;
}

export namespace Size {
  export type AsObject = {
    width: number,
    height: number,
  }
}

export class Coordinate extends jspb.Message {
  getX(): number;
  setX(value: number): Coordinate;

  getY(): number;
  setY(value: number): Coordinate;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Coordinate.AsObject;
  static toObject(includeInstance: boolean, msg: Coordinate): Coordinate.AsObject;
  static serializeBinaryToWriter(message: Coordinate, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Coordinate;
  static deserializeBinaryFromReader(message: Coordinate, reader: jspb.BinaryReader): Coordinate;
}

export namespace Coordinate {
  export type AsObject = {
    x: number,
    y: number,
  }
}

export class World extends jspb.Message {
  getSize(): Size | undefined;
  setSize(value?: Size): World;
  hasSize(): boolean;
  clearSize(): World;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): World.AsObject;
  static toObject(includeInstance: boolean, msg: World): World.AsObject;
  static serializeBinaryToWriter(message: World, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): World;
  static deserializeBinaryFromReader(message: World, reader: jspb.BinaryReader): World;
}

export namespace World {
  export type AsObject = {
    size?: Size.AsObject,
  }
}

export class GetWorldRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetWorldRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetWorldRequest): GetWorldRequest.AsObject;
  static serializeBinaryToWriter(message: GetWorldRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetWorldRequest;
  static deserializeBinaryFromReader(message: GetWorldRequest, reader: jspb.BinaryReader): GetWorldRequest;
}

export namespace GetWorldRequest {
  export type AsObject = {
  }
}

export class GetWorldResponse extends jspb.Message {
  getWorld(): World | undefined;
  setWorld(value?: World): GetWorldResponse;
  hasWorld(): boolean;
  clearWorld(): GetWorldResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetWorldResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetWorldResponse): GetWorldResponse.AsObject;
  static serializeBinaryToWriter(message: GetWorldResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetWorldResponse;
  static deserializeBinaryFromReader(message: GetWorldResponse, reader: jspb.BinaryReader): GetWorldResponse;
}

export namespace GetWorldResponse {
  export type AsObject = {
    world?: World.AsObject,
  }
}

