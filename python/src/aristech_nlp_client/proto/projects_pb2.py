# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: projects.proto
# Protobuf Python Version: 5.28.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    28,
    1,
    '',
    'projects.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0eprojects.proto\x12\x0c\x61ristech.nlp\"\x14\n\x12GetProjectsRequest\"#\n\x07Project\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\"\xd6\x02\n\x11\x41\x64\x64ProjectRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x02 \x01(\t\x12\x35\n\x0f\x65mbedding_model\x18\x03 \x01(\x0b\x32\x1c.aristech.nlp.EmbeddingModel\x12\x38\n\x11\x66\x61llback_messages\x18\x04 \x03(\x0b\x32\x1d.aristech.nlp.FallbackMessage\x12\x19\n\x11\x64\x65\x66\x61ult_threshold\x18\x05 \x01(\x02\x12\x0f\n\x07team_id\x18\x06 \x01(\t\x12\x12\n\ndebug_mode\x18\x07 \x01(\x08\x12\"\n\x1a\x65xclude_output_from_search\x18\x08 \x01(\x08\x12&\n\x07history\x18\t \x01(\x0b\x32\x15.aristech.nlp.History\x12\x13\n\x0b\x63onfig_slug\x18\n \x01(\t\x12\x0c\n\x04slug\x18\x0b \x01(\t\"|\n\x0f\x46\x61llbackMessage\x12\x0f\n\x07message\x18\x01 \x01(\t\x12\x30\n\x04type\x18\x02 \x01(\x0e\x32\".aristech.nlp.FallbackMessage.Type\"&\n\x04Type\x12\x08\n\x04\x43HAT\x10\x00\x12\t\n\x05\x45MAIL\x10\x01\x12\t\n\x05VOICE\x10\x02\"`\n\x07History\x12\x12\n\ncreator_id\x18\x01 \x01(\t\x12\x12\n\nchanged_by\x18\x02 \x01(\t\x12\x15\n\rcreation_date\x18\x03 \x01(\t\x12\x16\n\x0elast_edit_date\x18\x04 \x01(\t\"(\n\x12\x41\x64\x64ProjectResponse\x12\x12\n\nproject_id\x18\x01 \x01(\t\"*\n\x14RemoveProjectRequest\x12\x12\n\nproject_id\x18\x01 \x01(\t\"\x17\n\x15RemoveProjectResponse\"\x1b\n\x19GetEmbeddingModelsRequest\"X\n\x0e\x45mbeddingModel\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x12\n\ndimensions\x18\x02 \x01(\x05\x12\x14\n\x0c\x62\x61se_library\x18\x03 \x01(\t\x12\x0e\n\x06locale\x18\x04 \x03(\t\"\xdc\x02\n\x14UpdateProjectRequest\x12\x12\n\nproject_id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12\x35\n\x0f\x65mbedding_model\x18\x04 \x01(\x0b\x32\x1c.aristech.nlp.EmbeddingModel\x12\x38\n\x11\x66\x61llback_messages\x18\x05 \x03(\x0b\x32\x1d.aristech.nlp.FallbackMessage\x12\x19\n\x11\x64\x65\x66\x61ult_threshold\x18\x06 \x01(\x02\x12\x12\n\ndebug_mode\x18\x07 \x01(\x08\x12\"\n\x1a\x65xclude_output_from_search\x18\x08 \x01(\x08\x12&\n\x07history\x18\t \x01(\x0b\x32\x15.aristech.nlp.History\x12\x13\n\x0b\x63onfig_slug\x18\n \x01(\t\x12\x0c\n\x04slug\x18\x0b \x01(\t\"\x17\n\x15UpdateProjectResponseB\x08P\x01\xa2\x02\x03\x41TSb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'projects_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'P\001\242\002\003ATS'
  _globals['_GETPROJECTSREQUEST']._serialized_start=32
  _globals['_GETPROJECTSREQUEST']._serialized_end=52
  _globals['_PROJECT']._serialized_start=54
  _globals['_PROJECT']._serialized_end=89
  _globals['_ADDPROJECTREQUEST']._serialized_start=92
  _globals['_ADDPROJECTREQUEST']._serialized_end=434
  _globals['_FALLBACKMESSAGE']._serialized_start=436
  _globals['_FALLBACKMESSAGE']._serialized_end=560
  _globals['_FALLBACKMESSAGE_TYPE']._serialized_start=522
  _globals['_FALLBACKMESSAGE_TYPE']._serialized_end=560
  _globals['_HISTORY']._serialized_start=562
  _globals['_HISTORY']._serialized_end=658
  _globals['_ADDPROJECTRESPONSE']._serialized_start=660
  _globals['_ADDPROJECTRESPONSE']._serialized_end=700
  _globals['_REMOVEPROJECTREQUEST']._serialized_start=702
  _globals['_REMOVEPROJECTREQUEST']._serialized_end=744
  _globals['_REMOVEPROJECTRESPONSE']._serialized_start=746
  _globals['_REMOVEPROJECTRESPONSE']._serialized_end=769
  _globals['_GETEMBEDDINGMODELSREQUEST']._serialized_start=771
  _globals['_GETEMBEDDINGMODELSREQUEST']._serialized_end=798
  _globals['_EMBEDDINGMODEL']._serialized_start=800
  _globals['_EMBEDDINGMODEL']._serialized_end=888
  _globals['_UPDATEPROJECTREQUEST']._serialized_start=891
  _globals['_UPDATEPROJECTREQUEST']._serialized_end=1239
  _globals['_UPDATEPROJECTRESPONSE']._serialized_start=1241
  _globals['_UPDATEPROJECTRESPONSE']._serialized_end=1264
# @@protoc_insertion_point(module_scope)
