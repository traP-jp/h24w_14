// source: reaction.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
 
// @ts-nocheck

var jspb = require('google-protobuf');
var goog = jspb;
var global =
    (typeof globalThis !== 'undefined' && globalThis) ||
    (typeof window !== 'undefined' && window) ||
    (typeof global !== 'undefined' && global) ||
    (typeof self !== 'undefined' && self) ||
    (function () { return this; }).call(null) ||
    Function('return this')();

var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
goog.object.extend(proto, google_protobuf_timestamp_pb);
var world_pb = require('./world_pb.js');
goog.object.extend(proto, world_pb);
goog.exportSymbol('proto.reaction.CreateReactionRequest', null, global);
goog.exportSymbol('proto.reaction.CreateReactionResponse', null, global);
goog.exportSymbol('proto.reaction.GetReactionRequest', null, global);
goog.exportSymbol('proto.reaction.GetReactionResponse', null, global);
goog.exportSymbol('proto.reaction.Reaction', null, global);
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.reaction.Reaction = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.reaction.Reaction, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.reaction.Reaction.displayName = 'proto.reaction.Reaction';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.reaction.GetReactionRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.reaction.GetReactionRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.reaction.GetReactionRequest.displayName = 'proto.reaction.GetReactionRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.reaction.GetReactionResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.reaction.GetReactionResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.reaction.GetReactionResponse.displayName = 'proto.reaction.GetReactionResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.reaction.CreateReactionRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.reaction.CreateReactionRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.reaction.CreateReactionRequest.displayName = 'proto.reaction.CreateReactionRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.reaction.CreateReactionResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.reaction.CreateReactionResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.reaction.CreateReactionResponse.displayName = 'proto.reaction.CreateReactionResponse';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.reaction.Reaction.prototype.toObject = function(opt_includeInstance) {
  return proto.reaction.Reaction.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.reaction.Reaction} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.Reaction.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, ""),
userId: jspb.Message.getFieldWithDefault(msg, 2, ""),
position: (f = msg.getPosition()) && world_pb.Coordinate.toObject(includeInstance, f),
kind: jspb.Message.getFieldWithDefault(msg, 4, ""),
createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
expiresAt: (f = msg.getExpiresAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.reaction.Reaction}
 */
proto.reaction.Reaction.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.reaction.Reaction;
  return proto.reaction.Reaction.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.reaction.Reaction} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.reaction.Reaction}
 */
proto.reaction.Reaction.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setUserId(value);
      break;
    case 3:
      var value = new world_pb.Coordinate;
      reader.readMessage(value,world_pb.Coordinate.deserializeBinaryFromReader);
      msg.setPosition(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setKind(value);
      break;
    case 5:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    case 6:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setExpiresAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.reaction.Reaction.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.reaction.Reaction.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.reaction.Reaction} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.Reaction.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getUserId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getPosition();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      world_pb.Coordinate.serializeBinaryToWriter
    );
  }
  f = message.getKind();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getExpiresAt();
  if (f != null) {
    writer.writeMessage(
      6,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.reaction.Reaction.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.reaction.Reaction} returns this
 */
proto.reaction.Reaction.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string user_id = 2;
 * @return {string}
 */
proto.reaction.Reaction.prototype.getUserId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.reaction.Reaction} returns this
 */
proto.reaction.Reaction.prototype.setUserId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional world.Coordinate position = 3;
 * @return {?proto.world.Coordinate}
 */
proto.reaction.Reaction.prototype.getPosition = function() {
  return /** @type{?proto.world.Coordinate} */ (
    jspb.Message.getWrapperField(this, world_pb.Coordinate, 3));
};


/**
 * @param {?proto.world.Coordinate|undefined} value
 * @return {!proto.reaction.Reaction} returns this
*/
proto.reaction.Reaction.prototype.setPosition = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.reaction.Reaction} returns this
 */
proto.reaction.Reaction.prototype.clearPosition = function() {
  return this.setPosition(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.reaction.Reaction.prototype.hasPosition = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional string kind = 4;
 * @return {string}
 */
proto.reaction.Reaction.prototype.getKind = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.reaction.Reaction} returns this
 */
proto.reaction.Reaction.prototype.setKind = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional google.protobuf.Timestamp created_at = 5;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.reaction.Reaction.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 5));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.reaction.Reaction} returns this
*/
proto.reaction.Reaction.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 5, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.reaction.Reaction} returns this
 */
proto.reaction.Reaction.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.reaction.Reaction.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional google.protobuf.Timestamp expires_at = 6;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.reaction.Reaction.prototype.getExpiresAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 6));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.reaction.Reaction} returns this
*/
proto.reaction.Reaction.prototype.setExpiresAt = function(value) {
  return jspb.Message.setWrapperField(this, 6, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.reaction.Reaction} returns this
 */
proto.reaction.Reaction.prototype.clearExpiresAt = function() {
  return this.setExpiresAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.reaction.Reaction.prototype.hasExpiresAt = function() {
  return jspb.Message.getField(this, 6) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.reaction.GetReactionRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.reaction.GetReactionRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.reaction.GetReactionRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.GetReactionRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.reaction.GetReactionRequest}
 */
proto.reaction.GetReactionRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.reaction.GetReactionRequest;
  return proto.reaction.GetReactionRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.reaction.GetReactionRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.reaction.GetReactionRequest}
 */
proto.reaction.GetReactionRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.reaction.GetReactionRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.reaction.GetReactionRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.reaction.GetReactionRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.GetReactionRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.reaction.GetReactionRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.reaction.GetReactionRequest} returns this
 */
proto.reaction.GetReactionRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.reaction.GetReactionResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.reaction.GetReactionResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.reaction.GetReactionResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.GetReactionResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
reaction: (f = msg.getReaction()) && proto.reaction.Reaction.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.reaction.GetReactionResponse}
 */
proto.reaction.GetReactionResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.reaction.GetReactionResponse;
  return proto.reaction.GetReactionResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.reaction.GetReactionResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.reaction.GetReactionResponse}
 */
proto.reaction.GetReactionResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.reaction.Reaction;
      reader.readMessage(value,proto.reaction.Reaction.deserializeBinaryFromReader);
      msg.setReaction(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.reaction.GetReactionResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.reaction.GetReactionResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.reaction.GetReactionResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.GetReactionResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getReaction();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.reaction.Reaction.serializeBinaryToWriter
    );
  }
};


/**
 * optional Reaction reaction = 1;
 * @return {?proto.reaction.Reaction}
 */
proto.reaction.GetReactionResponse.prototype.getReaction = function() {
  return /** @type{?proto.reaction.Reaction} */ (
    jspb.Message.getWrapperField(this, proto.reaction.Reaction, 1));
};


/**
 * @param {?proto.reaction.Reaction|undefined} value
 * @return {!proto.reaction.GetReactionResponse} returns this
*/
proto.reaction.GetReactionResponse.prototype.setReaction = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.reaction.GetReactionResponse} returns this
 */
proto.reaction.GetReactionResponse.prototype.clearReaction = function() {
  return this.setReaction(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.reaction.GetReactionResponse.prototype.hasReaction = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.reaction.CreateReactionRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.reaction.CreateReactionRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.reaction.CreateReactionRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.CreateReactionRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
position: (f = msg.getPosition()) && world_pb.Coordinate.toObject(includeInstance, f),
kind: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.reaction.CreateReactionRequest}
 */
proto.reaction.CreateReactionRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.reaction.CreateReactionRequest;
  return proto.reaction.CreateReactionRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.reaction.CreateReactionRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.reaction.CreateReactionRequest}
 */
proto.reaction.CreateReactionRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 2:
      var value = new world_pb.Coordinate;
      reader.readMessage(value,world_pb.Coordinate.deserializeBinaryFromReader);
      msg.setPosition(value);
      break;
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKind(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.reaction.CreateReactionRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.reaction.CreateReactionRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.reaction.CreateReactionRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.CreateReactionRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPosition();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      world_pb.Coordinate.serializeBinaryToWriter
    );
  }
  f = message.getKind();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional world.Coordinate position = 2;
 * @return {?proto.world.Coordinate}
 */
proto.reaction.CreateReactionRequest.prototype.getPosition = function() {
  return /** @type{?proto.world.Coordinate} */ (
    jspb.Message.getWrapperField(this, world_pb.Coordinate, 2));
};


/**
 * @param {?proto.world.Coordinate|undefined} value
 * @return {!proto.reaction.CreateReactionRequest} returns this
*/
proto.reaction.CreateReactionRequest.prototype.setPosition = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.reaction.CreateReactionRequest} returns this
 */
proto.reaction.CreateReactionRequest.prototype.clearPosition = function() {
  return this.setPosition(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.reaction.CreateReactionRequest.prototype.hasPosition = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional string kind = 1;
 * @return {string}
 */
proto.reaction.CreateReactionRequest.prototype.getKind = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.reaction.CreateReactionRequest} returns this
 */
proto.reaction.CreateReactionRequest.prototype.setKind = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.reaction.CreateReactionResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.reaction.CreateReactionResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.reaction.CreateReactionResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.CreateReactionResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
reaction: (f = msg.getReaction()) && proto.reaction.Reaction.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.reaction.CreateReactionResponse}
 */
proto.reaction.CreateReactionResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.reaction.CreateReactionResponse;
  return proto.reaction.CreateReactionResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.reaction.CreateReactionResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.reaction.CreateReactionResponse}
 */
proto.reaction.CreateReactionResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.reaction.Reaction;
      reader.readMessage(value,proto.reaction.Reaction.deserializeBinaryFromReader);
      msg.setReaction(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.reaction.CreateReactionResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.reaction.CreateReactionResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.reaction.CreateReactionResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.reaction.CreateReactionResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getReaction();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.reaction.Reaction.serializeBinaryToWriter
    );
  }
};


/**
 * optional Reaction reaction = 1;
 * @return {?proto.reaction.Reaction}
 */
proto.reaction.CreateReactionResponse.prototype.getReaction = function() {
  return /** @type{?proto.reaction.Reaction} */ (
    jspb.Message.getWrapperField(this, proto.reaction.Reaction, 1));
};


/**
 * @param {?proto.reaction.Reaction|undefined} value
 * @return {!proto.reaction.CreateReactionResponse} returns this
*/
proto.reaction.CreateReactionResponse.prototype.setReaction = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.reaction.CreateReactionResponse} returns this
 */
proto.reaction.CreateReactionResponse.prototype.clearReaction = function() {
  return this.setReaction(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.reaction.CreateReactionResponse.prototype.hasReaction = function() {
  return jspb.Message.getField(this, 1) != null;
};


goog.object.extend(exports, proto.reaction);
