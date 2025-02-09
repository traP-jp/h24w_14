// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "auth.proto" (package "auth", syntax proto3)
// tslint:disable
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message auth.AuthRequest
 */
export interface AuthRequest {
}
/**
 * @generated from protobuf message auth.AuthResponse
 */
export interface AuthResponse {
    /**
     * リダイレクト先
     *
     * @generated from protobuf field: string location = 1;
     */
    location: string;
}
// @generated message type with reflection information, may provide speed optimized methods
class AuthRequest$Type extends MessageType<AuthRequest> {
    constructor() {
        super("auth.AuthRequest", []);
    }
    create(value?: PartialMessage<AuthRequest>): AuthRequest {
        const message = globalThis.Object.create((this.messagePrototype!));
        if (value !== undefined)
            reflectionMergePartial<AuthRequest>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: AuthRequest): AuthRequest {
        return target ?? this.create();
    }
    internalBinaryWrite(message: AuthRequest, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message auth.AuthRequest
 */
export const AuthRequest = new AuthRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class AuthResponse$Type extends MessageType<AuthResponse> {
    constructor() {
        super("auth.AuthResponse", [
            { no: 1, name: "location", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
    create(value?: PartialMessage<AuthResponse>): AuthResponse {
        const message = globalThis.Object.create((this.messagePrototype!));
        message.location = "";
        if (value !== undefined)
            reflectionMergePartial<AuthResponse>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: AuthResponse): AuthResponse {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* string location */ 1:
                    message.location = reader.string();
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
    internalBinaryWrite(message: AuthResponse, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* string location = 1; */
        if (message.location !== "")
            writer.tag(1, WireType.LengthDelimited).string(message.location);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message auth.AuthResponse
 */
export const AuthResponse = new AuthResponse$Type();
/**
 * @generated ServiceType for protobuf service auth.AuthService
 */
export const AuthService = new ServiceType("auth.AuthService", [
    { name: "Auth", options: {}, I: AuthRequest, O: AuthResponse }
]);
