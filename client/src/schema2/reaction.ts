// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "reaction.proto" (package "reaction", syntax proto3)
// tslint:disable
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
import { Timestamp } from "./google/protobuf/timestamp";
import { Coordinate } from "./world";
/**
 * @generated from protobuf message reaction.Reaction
 */
export interface Reaction {
    /**
     * UUID
     *
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * リアクションをしたユーザーのID
     *
     * @generated from protobuf field: string user_id = 2;
     */
    userId: string;
    /**
     * リアクションをした座標
     *
     * @generated from protobuf field: world.Coordinate position = 3;
     */
    position?: Coordinate;
    /**
     * リアクションの種類
     *
     * @generated from protobuf field: string kind = 4;
     */
    kind: string;
    /**
     * リアクションをした日時
     *
     * @generated from protobuf field: google.protobuf.Timestamp created_at = 5;
     */
    createdAt?: Timestamp;
    /**
     * ユーザーがアクセスできる期限
     * TODO: このフィールドは必要か？
     *
     * @generated from protobuf field: google.protobuf.Timestamp expires_at = 6;
     */
    expiresAt?: Timestamp;
}
/**
 * @generated from protobuf message reaction.GetReactionRequest
 */
export interface GetReactionRequest {
    /**
     * @generated from protobuf field: string id = 1;
     */
    id: string;
}
/**
 * @generated from protobuf message reaction.GetReactionResponse
 */
export interface GetReactionResponse {
    /**
     * @generated from protobuf field: reaction.Reaction reaction = 1;
     */
    reaction?: Reaction;
}
/**
 * リアクションの作成
 *
 * @generated from protobuf message reaction.CreateReactionRequest
 */
export interface CreateReactionRequest {
    /**
     * リアクションをする座標
     *
     * @generated from protobuf field: world.Coordinate position = 2;
     */
    position?: Coordinate;
    /**
     * リアクションの種類
     *
     * @generated from protobuf field: string kind = 1;
     */
    kind: string;
}
/**
 * @generated from protobuf message reaction.CreateReactionResponse
 */
export interface CreateReactionResponse {
    /**
     * @generated from protobuf field: reaction.Reaction reaction = 1;
     */
    reaction?: Reaction;
}
// @generated message type with reflection information, may provide speed optimized methods
class Reaction$Type extends MessageType<Reaction> {
    constructor() {
        super("reaction.Reaction", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "user_id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "position", kind: "message", T: () => Coordinate },
            { no: 4, name: "kind", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 5, name: "created_at", kind: "message", T: () => Timestamp },
            { no: 6, name: "expires_at", kind: "message", T: () => Timestamp }
        ]);
    }
    create(value?: PartialMessage<Reaction>): Reaction {
        const message = globalThis.Object.create((this.messagePrototype!));
        message.id = "";
        message.userId = "";
        message.kind = "";
        if (value !== undefined)
            reflectionMergePartial<Reaction>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: Reaction): Reaction {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string id */ 1:
                    message.id = reader.string();
                    break;
                case /* string user_id */ 2:
                    message.userId = reader.string();
                    break;
                case /* world.Coordinate position */ 3:
                    message.position = Coordinate.internalBinaryRead(reader, reader.uint32(), options, message.position);
                    break;
                case /* string kind */ 4:
                    message.kind = reader.string();
                    break;
                case /* google.protobuf.Timestamp created_at */ 5:
                    message.createdAt = Timestamp.internalBinaryRead(reader, reader.uint32(), options, message.createdAt);
                    break;
                case /* google.protobuf.Timestamp expires_at */ 6:
                    message.expiresAt = Timestamp.internalBinaryRead(reader, reader.uint32(), options, message.expiresAt);
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: Reaction, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string id = 1; */
        if (message.id !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.id);
        /* string user_id = 2; */
        if (message.userId !== "")
            writer.tag(2, WireType.LengthDelimited).string(message.userId);
        /* world.Coordinate position = 3; */
        if (message.position)
            Coordinate.internalBinaryWrite(message.position, writer.tag(3, WireType.LengthDelimited).fork(), options).join();
        /* string kind = 4; */
        if (message.kind !== "")
            writer.tag(4, WireType.LengthDelimited).string(message.kind);
        /* google.protobuf.Timestamp created_at = 5; */
        if (message.createdAt)
            Timestamp.internalBinaryWrite(message.createdAt, writer.tag(5, WireType.LengthDelimited).fork(), options).join();
        /* google.protobuf.Timestamp expires_at = 6; */
        if (message.expiresAt)
            Timestamp.internalBinaryWrite(message.expiresAt, writer.tag(6, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message reaction.Reaction
 */
export const Reaction = new Reaction$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetReactionRequest$Type extends MessageType<GetReactionRequest> {
    constructor() {
        super("reaction.GetReactionRequest", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value?: PartialMessage<GetReactionRequest>): GetReactionRequest {
        const message = globalThis.Object.create((this.messagePrototype!));
        message.id = "";
        if (value !== undefined)
            reflectionMergePartial<GetReactionRequest>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: GetReactionRequest): GetReactionRequest {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string id */ 1:
                    message.id = reader.string();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: GetReactionRequest, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string id = 1; */
        if (message.id !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.id);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message reaction.GetReactionRequest
 */
export const GetReactionRequest = new GetReactionRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetReactionResponse$Type extends MessageType<GetReactionResponse> {
    constructor() {
        super("reaction.GetReactionResponse", [
            { no: 1, name: "reaction", kind: "message", T: () => Reaction }
        ]);
    }
    create(value?: PartialMessage<GetReactionResponse>): GetReactionResponse {
        const message = globalThis.Object.create((this.messagePrototype!));
        if (value !== undefined)
            reflectionMergePartial<GetReactionResponse>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: GetReactionResponse): GetReactionResponse {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* reaction.Reaction reaction */ 1:
                    message.reaction = Reaction.internalBinaryRead(reader, reader.uint32(), options, message.reaction);
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: GetReactionResponse, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* reaction.Reaction reaction = 1; */
        if (message.reaction)
            Reaction.internalBinaryWrite(message.reaction, writer.tag(1, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message reaction.GetReactionResponse
 */
export const GetReactionResponse = new GetReactionResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class CreateReactionRequest$Type extends MessageType<CreateReactionRequest> {
    constructor() {
        super("reaction.CreateReactionRequest", [
            { no: 2, name: "position", kind: "message", T: () => Coordinate },
            { no: 1, name: "kind", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value?: PartialMessage<CreateReactionRequest>): CreateReactionRequest {
        const message = globalThis.Object.create((this.messagePrototype!));
        message.kind = "";
        if (value !== undefined)
            reflectionMergePartial<CreateReactionRequest>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: CreateReactionRequest): CreateReactionRequest {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* world.Coordinate position */ 2:
                    message.position = Coordinate.internalBinaryRead(reader, reader.uint32(), options, message.position);
                    break;
                case /* string kind */ 1:
                    message.kind = reader.string();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: CreateReactionRequest, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* world.Coordinate position = 2; */
        if (message.position)
            Coordinate.internalBinaryWrite(message.position, writer.tag(2, WireType.LengthDelimited).fork(), options).join();
        /* string kind = 1; */
        if (message.kind !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.kind);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message reaction.CreateReactionRequest
 */
export const CreateReactionRequest = new CreateReactionRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class CreateReactionResponse$Type extends MessageType<CreateReactionResponse> {
    constructor() {
        super("reaction.CreateReactionResponse", [
            { no: 1, name: "reaction", kind: "message", T: () => Reaction }
        ]);
    }
    create(value?: PartialMessage<CreateReactionResponse>): CreateReactionResponse {
        const message = globalThis.Object.create((this.messagePrototype!));
        if (value !== undefined)
            reflectionMergePartial<CreateReactionResponse>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: CreateReactionResponse): CreateReactionResponse {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* reaction.Reaction reaction */ 1:
                    message.reaction = Reaction.internalBinaryRead(reader, reader.uint32(), options, message.reaction);
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: CreateReactionResponse, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* reaction.Reaction reaction = 1; */
        if (message.reaction)
            Reaction.internalBinaryWrite(message.reaction, writer.tag(1, WireType.LengthDelimited).fork(), options).join();
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message reaction.CreateReactionResponse
 */
export const CreateReactionResponse = new CreateReactionResponse$Type();
/**
 * @generated ServiceType for protobuf service reaction.ReactionService
 */
export const ReactionService = new ServiceType("reaction.ReactionService", [
    { name: "GetReaction", options: {}, I: GetReactionRequest, O: GetReactionResponse },
    { name: "CreateReaction", options: {}, I: CreateReactionRequest, O: CreateReactionResponse }
]);
