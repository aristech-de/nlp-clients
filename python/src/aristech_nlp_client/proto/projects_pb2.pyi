"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
Copyright Aristech GmbH"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class GetProjectsRequest(google.protobuf.message.Message):
    """Methods and datatypes for (vector) content management"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___GetProjectsRequest = GetProjectsRequest

@typing.final
class Project(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    id: builtins.str
    """ID of the project"""
    name: builtins.str
    """Name of the project"""
    def __init__(
        self,
        *,
        id: builtins.str = ...,
        name: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["id", b"id", "name", b"name"]) -> None: ...

global___Project = Project

@typing.final
class AddProjectRequest(google.protobuf.message.Message):
    """adds a project to the underlying database(s)"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    EMBEDDING_MODEL_FIELD_NUMBER: builtins.int
    FALLBACK_MESSAGES_FIELD_NUMBER: builtins.int
    DEFAULT_THRESHOLD_FIELD_NUMBER: builtins.int
    TEAM_ID_FIELD_NUMBER: builtins.int
    DEBUG_MODE_FIELD_NUMBER: builtins.int
    EXCLUDE_OUTPUT_FROM_SEARCH_FIELD_NUMBER: builtins.int
    HISTORY_FIELD_NUMBER: builtins.int
    CONFIG_SLUG_FIELD_NUMBER: builtins.int
    SLUG_FIELD_NUMBER: builtins.int
    name: builtins.str
    """Project name"""
    description: builtins.str
    """description of the project"""
    default_threshold: builtins.float
    """default_threshold for search requests"""
    team_id: builtins.str
    debug_mode: builtins.bool
    exclude_output_from_search: builtins.bool
    config_slug: builtins.str
    slug: builtins.str
    @property
    def embedding_model(self) -> global___EmbeddingModel:
        """model to use for vectorembeddings"""

    @property
    def fallback_messages(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___FallbackMessage]:
        """Fallback Messages"""

    @property
    def history(self) -> global___History: ...
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        description: builtins.str = ...,
        embedding_model: global___EmbeddingModel | None = ...,
        fallback_messages: collections.abc.Iterable[global___FallbackMessage] | None = ...,
        default_threshold: builtins.float = ...,
        team_id: builtins.str = ...,
        debug_mode: builtins.bool = ...,
        exclude_output_from_search: builtins.bool = ...,
        history: global___History | None = ...,
        config_slug: builtins.str = ...,
        slug: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["embedding_model", b"embedding_model", "history", b"history"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["config_slug", b"config_slug", "debug_mode", b"debug_mode", "default_threshold", b"default_threshold", "description", b"description", "embedding_model", b"embedding_model", "exclude_output_from_search", b"exclude_output_from_search", "fallback_messages", b"fallback_messages", "history", b"history", "name", b"name", "slug", b"slug", "team_id", b"team_id"]) -> None: ...

global___AddProjectRequest = AddProjectRequest

@typing.final
class FallbackMessage(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    class _Type:
        ValueType = typing.NewType("ValueType", builtins.int)
        V: typing_extensions.TypeAlias = ValueType

    class _TypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[FallbackMessage._Type.ValueType], builtins.type):
        DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
        CHAT: FallbackMessage._Type.ValueType  # 0
        EMAIL: FallbackMessage._Type.ValueType  # 1
        VOICE: FallbackMessage._Type.ValueType  # 2

    class Type(_Type, metaclass=_TypeEnumTypeWrapper): ...
    CHAT: FallbackMessage.Type.ValueType  # 0
    EMAIL: FallbackMessage.Type.ValueType  # 1
    VOICE: FallbackMessage.Type.ValueType  # 2

    MESSAGE_FIELD_NUMBER: builtins.int
    TYPE_FIELD_NUMBER: builtins.int
    message: builtins.str
    type: global___FallbackMessage.Type.ValueType
    def __init__(
        self,
        *,
        message: builtins.str = ...,
        type: global___FallbackMessage.Type.ValueType = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["message", b"message", "type", b"type"]) -> None: ...

global___FallbackMessage = FallbackMessage

@typing.final
class History(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CREATOR_ID_FIELD_NUMBER: builtins.int
    CHANGED_BY_FIELD_NUMBER: builtins.int
    CREATION_DATE_FIELD_NUMBER: builtins.int
    LAST_EDIT_DATE_FIELD_NUMBER: builtins.int
    creator_id: builtins.str
    """object id with the creator id"""
    changed_by: builtins.str
    """id of the user who made the last change"""
    creation_date: builtins.str
    """Date of creation"""
    last_edit_date: builtins.str
    """date of last edit"""
    def __init__(
        self,
        *,
        creator_id: builtins.str = ...,
        changed_by: builtins.str = ...,
        creation_date: builtins.str = ...,
        last_edit_date: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["changed_by", b"changed_by", "creation_date", b"creation_date", "creator_id", b"creator_id", "last_edit_date", b"last_edit_date"]) -> None: ...

global___History = History

@typing.final
class AddProjectResponse(google.protobuf.message.Message):
    """response for adding a project. returns the project_id of the created project"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PROJECT_ID_FIELD_NUMBER: builtins.int
    project_id: builtins.str
    """id of the created project"""
    def __init__(
        self,
        *,
        project_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["project_id", b"project_id"]) -> None: ...

global___AddProjectResponse = AddProjectResponse

@typing.final
class RemoveProjectRequest(google.protobuf.message.Message):
    """removes the project and all of its content"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PROJECT_ID_FIELD_NUMBER: builtins.int
    project_id: builtins.str
    """ID of the project to be removed"""
    def __init__(
        self,
        *,
        project_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["project_id", b"project_id"]) -> None: ...

global___RemoveProjectRequest = RemoveProjectRequest

@typing.final
class RemoveProjectResponse(google.protobuf.message.Message):
    """response for remove project request"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___RemoveProjectResponse = RemoveProjectResponse

@typing.final
class GetEmbeddingModelsRequest(google.protobuf.message.Message):
    """request to get all available embedding models from the server"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___GetEmbeddingModelsRequest = GetEmbeddingModelsRequest

@typing.final
class EmbeddingModel(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    DIMENSIONS_FIELD_NUMBER: builtins.int
    BASE_LIBRARY_FIELD_NUMBER: builtins.int
    LOCALE_FIELD_NUMBER: builtins.int
    name: builtins.str
    dimensions: builtins.int
    base_library: builtins.str
    @property
    def locale(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        dimensions: builtins.int = ...,
        base_library: builtins.str = ...,
        locale: collections.abc.Iterable[builtins.str] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["base_library", b"base_library", "dimensions", b"dimensions", "locale", b"locale", "name", b"name"]) -> None: ...

global___EmbeddingModel = EmbeddingModel

@typing.final
class UpdateProjectRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PROJECT_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    EMBEDDING_MODEL_FIELD_NUMBER: builtins.int
    FALLBACK_MESSAGES_FIELD_NUMBER: builtins.int
    DEFAULT_THRESHOLD_FIELD_NUMBER: builtins.int
    DEBUG_MODE_FIELD_NUMBER: builtins.int
    EXCLUDE_OUTPUT_FROM_SEARCH_FIELD_NUMBER: builtins.int
    HISTORY_FIELD_NUMBER: builtins.int
    CONFIG_SLUG_FIELD_NUMBER: builtins.int
    SLUG_FIELD_NUMBER: builtins.int
    project_id: builtins.str
    """the id of the project to be updated, if no id is set, a new project is created"""
    name: builtins.str
    """Project name"""
    description: builtins.str
    """description of the project"""
    default_threshold: builtins.float
    """default_threshold for search requests"""
    debug_mode: builtins.bool
    exclude_output_from_search: builtins.bool
    config_slug: builtins.str
    slug: builtins.str
    @property
    def embedding_model(self) -> global___EmbeddingModel:
        """model to use for vectorembeddings"""

    @property
    def fallback_messages(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___FallbackMessage]:
        """Fallback Messages"""

    @property
    def history(self) -> global___History: ...
    def __init__(
        self,
        *,
        project_id: builtins.str = ...,
        name: builtins.str = ...,
        description: builtins.str = ...,
        embedding_model: global___EmbeddingModel | None = ...,
        fallback_messages: collections.abc.Iterable[global___FallbackMessage] | None = ...,
        default_threshold: builtins.float = ...,
        debug_mode: builtins.bool = ...,
        exclude_output_from_search: builtins.bool = ...,
        history: global___History | None = ...,
        config_slug: builtins.str = ...,
        slug: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["embedding_model", b"embedding_model", "history", b"history"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["config_slug", b"config_slug", "debug_mode", b"debug_mode", "default_threshold", b"default_threshold", "description", b"description", "embedding_model", b"embedding_model", "exclude_output_from_search", b"exclude_output_from_search", "fallback_messages", b"fallback_messages", "history", b"history", "name", b"name", "project_id", b"project_id", "slug", b"slug"]) -> None: ...

global___UpdateProjectRequest = UpdateProjectRequest

@typing.final
class UpdateProjectResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___UpdateProjectResponse = UpdateProjectResponse
