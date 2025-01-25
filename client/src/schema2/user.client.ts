// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "user.proto" (package "user", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { UserService } from "./user";
import type { GetMeResponse } from "./user";
import type { GetMeRequest } from "./user";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { GetUserResponse } from "./user";
import type { GetUserRequest } from "./user";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service user.UserService
 */
export interface IUserServiceClient {
    /**
     * @generated from protobuf rpc: GetUser(user.GetUserRequest) returns (user.GetUserResponse);
     */
    getUser(input: GetUserRequest, options?: RpcOptions): UnaryCall<GetUserRequest, GetUserResponse>;
    /**
     * @generated from protobuf rpc: GetMe(user.GetMeRequest) returns (user.GetMeResponse);
     */
    getMe(input: GetMeRequest, options?: RpcOptions): UnaryCall<GetMeRequest, GetMeResponse>;
}
/**
 * @generated from protobuf service user.UserService
 */
export class UserServiceClient implements IUserServiceClient, ServiceInfo {
    typeName = UserService.typeName;
    methods = UserService.methods;
    options = UserService.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: GetUser(user.GetUserRequest) returns (user.GetUserResponse);
     */
    getUser(input: GetUserRequest, options?: RpcOptions): UnaryCall<GetUserRequest, GetUserResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetUserRequest, GetUserResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: GetMe(user.GetMeRequest) returns (user.GetMeResponse);
     */
    getMe(input: GetMeRequest, options?: RpcOptions): UnaryCall<GetMeRequest, GetMeResponse> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetMeRequest, GetMeResponse>("unary", this._transport, method, opt, input);
    }
}
