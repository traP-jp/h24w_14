import * as jspb from 'google-protobuf'

import * as message_pb from './message_pb'; // proto import: "message.proto"
import * as reaction_pb from './reaction_pb'; // proto import: "reaction.proto"
import * as speaker_phone_pb from './speaker_phone_pb'; // proto import: "speaker_phone.proto"
import * as world_pb from './world_pb'; // proto import: "world.proto"


export class Explorer extends jspb.Message {
  getId(): string;
  setId(value: string): Explorer;

  getUserId(): string;
  setUserId(value: string): Explorer;

  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): Explorer;
  hasPosition(): boolean;
  clearPosition(): Explorer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Explorer.AsObject;
  static toObject(includeInstance: boolean, msg: Explorer): Explorer.AsObject;
  static serializeBinaryToWriter(message: Explorer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Explorer;
  static deserializeBinaryFromReader(message: Explorer, reader: jspb.BinaryReader): Explorer;
}

export namespace Explorer {
  export type AsObject = {
    id: string,
    userId: string,
    position?: world_pb.Coordinate.AsObject,
  }
}

export class ExplorationField extends jspb.Message {
  getPosition(): world_pb.Coordinate | undefined;
  setPosition(value?: world_pb.Coordinate): ExplorationField;
  hasPosition(): boolean;
  clearPosition(): ExplorationField;

  getSize(): world_pb.Size | undefined;
  setSize(value?: world_pb.Size): ExplorationField;
  hasSize(): boolean;
  clearSize(): ExplorationField;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExplorationField.AsObject;
  static toObject(includeInstance: boolean, msg: ExplorationField): ExplorationField.AsObject;
  static serializeBinaryToWriter(message: ExplorationField, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExplorationField;
  static deserializeBinaryFromReader(message: ExplorationField, reader: jspb.BinaryReader): ExplorationField;
}

export namespace ExplorationField {
  export type AsObject = {
    position?: world_pb.Coordinate.AsObject,
    size?: world_pb.Size.AsObject,
  }
}

export class ExplorerAction extends jspb.Message {
  getArrive(): ExplorerAction.Arrive | undefined;
  setArrive(value?: ExplorerAction.Arrive): ExplorerAction;
  hasArrive(): boolean;
  clearArrive(): ExplorerAction;

  getMove(): ExplorerAction.Move | undefined;
  setMove(value?: ExplorerAction.Move): ExplorerAction;
  hasMove(): boolean;
  clearMove(): ExplorerAction;

  getLeave(): ExplorerAction.Leave | undefined;
  setLeave(value?: ExplorerAction.Leave): ExplorerAction;
  hasLeave(): boolean;
  clearLeave(): ExplorerAction;

  getActionCase(): ExplorerAction.ActionCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExplorerAction.AsObject;
  static toObject(includeInstance: boolean, msg: ExplorerAction): ExplorerAction.AsObject;
  static serializeBinaryToWriter(message: ExplorerAction, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExplorerAction;
  static deserializeBinaryFromReader(message: ExplorerAction, reader: jspb.BinaryReader): ExplorerAction;
}

export namespace ExplorerAction {
  export type AsObject = {
    arrive?: ExplorerAction.Arrive.AsObject,
    move?: ExplorerAction.Move.AsObject,
    leave?: ExplorerAction.Leave.AsObject,
  }

  export class Arrive extends jspb.Message {
    getExplorer(): Explorer | undefined;
    setExplorer(value?: Explorer): Arrive;
    hasExplorer(): boolean;
    clearExplorer(): Arrive;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Arrive.AsObject;
    static toObject(includeInstance: boolean, msg: Arrive): Arrive.AsObject;
    static serializeBinaryToWriter(message: Arrive, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Arrive;
    static deserializeBinaryFromReader(message: Arrive, reader: jspb.BinaryReader): Arrive;
  }

  export namespace Arrive {
    export type AsObject = {
      explorer?: Explorer.AsObject,
    }
  }


  export class Move extends jspb.Message {
    getExplorer(): Explorer | undefined;
    setExplorer(value?: Explorer): Move;
    hasExplorer(): boolean;
    clearExplorer(): Move;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Move.AsObject;
    static toObject(includeInstance: boolean, msg: Move): Move.AsObject;
    static serializeBinaryToWriter(message: Move, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Move;
    static deserializeBinaryFromReader(message: Move, reader: jspb.BinaryReader): Move;
  }

  export namespace Move {
    export type AsObject = {
      explorer?: Explorer.AsObject,
    }
  }


  export class Leave extends jspb.Message {
    getId(): string;
    setId(value: string): Leave;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Leave.AsObject;
    static toObject(includeInstance: boolean, msg: Leave): Leave.AsObject;
    static serializeBinaryToWriter(message: Leave, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Leave;
    static deserializeBinaryFromReader(message: Leave, reader: jspb.BinaryReader): Leave;
  }

  export namespace Leave {
    export type AsObject = {
      id: string,
    }
  }


  export enum ActionCase { 
    ACTION_NOT_SET = 0,
    ARRIVE = 1,
    MOVE = 2,
    LEAVE = 3,
  }
}

export class ExplorationFieldEvents extends jspb.Message {
  getMessagesList(): Array<message_pb.Message>;
  setMessagesList(value: Array<message_pb.Message>): ExplorationFieldEvents;
  clearMessagesList(): ExplorationFieldEvents;
  addMessages(value?: message_pb.Message, index?: number): message_pb.Message;

  getSpeakerPhonesList(): Array<speaker_phone_pb.SpeakerPhone>;
  setSpeakerPhonesList(value: Array<speaker_phone_pb.SpeakerPhone>): ExplorationFieldEvents;
  clearSpeakerPhonesList(): ExplorationFieldEvents;
  addSpeakerPhones(value?: speaker_phone_pb.SpeakerPhone, index?: number): speaker_phone_pb.SpeakerPhone;

  getReactionsList(): Array<reaction_pb.Reaction>;
  setReactionsList(value: Array<reaction_pb.Reaction>): ExplorationFieldEvents;
  clearReactionsList(): ExplorationFieldEvents;
  addReactions(value?: reaction_pb.Reaction, index?: number): reaction_pb.Reaction;

  getExplorerActionsList(): Array<ExplorerAction>;
  setExplorerActionsList(value: Array<ExplorerAction>): ExplorationFieldEvents;
  clearExplorerActionsList(): ExplorationFieldEvents;
  addExplorerActions(value?: ExplorerAction, index?: number): ExplorerAction;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExplorationFieldEvents.AsObject;
  static toObject(includeInstance: boolean, msg: ExplorationFieldEvents): ExplorationFieldEvents.AsObject;
  static serializeBinaryToWriter(message: ExplorationFieldEvents, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExplorationFieldEvents;
  static deserializeBinaryFromReader(message: ExplorationFieldEvents, reader: jspb.BinaryReader): ExplorationFieldEvents;
}

export namespace ExplorationFieldEvents {
  export type AsObject = {
    messagesList: Array<message_pb.Message.AsObject>,
    speakerPhonesList: Array<speaker_phone_pb.SpeakerPhone.AsObject>,
    reactionsList: Array<reaction_pb.Reaction.AsObject>,
    explorerActionsList: Array<ExplorerAction.AsObject>,
  }
}

