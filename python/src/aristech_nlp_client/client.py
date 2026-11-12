import grpc
import re

from typing import Iterable

from .proto.nlp_server_pb2_grpc import *
from .proto.nlp_server_pb2 import *
from .proto.projects_pb2_grpc import *
from .proto.projects_pb2 import *
from .proto.intents_pb2_grpc import *
from .proto.intents_pb2 import *
from .proto.documents_pb2 import *

class NlpClient:
    host: str
    ssl: bool
    root_cert: bytes
    auth_token: str
    auth_secret: str
    channel: grpc.Channel

    def __init__(self, host: str, ssl=None, root_cert=b"", auth_token="", auth_secret=""):
      """
      Initializes the client with the given connection parameters.

      Args:
        host (str): The host to connect to. Can include the port, e.g. "localhost:8524".
        ssl (bool): Whether to use SSL. If not explicitly set, the client will try to guess based on the remaining parameters.
        root_cert (bytes): The root certificate to use for SSL (e.g. when connecting to a server that uses a self-signed certificate).
        auth_token (str): The auth token to use for authentication.
        auth_secret (str): The auth secret to use for authentication.
      """
      # When ssl or root_cert are not explicitly set, we check if the host includes the port 9424 or 9423.
      # If host does not include the port, we assume ssl is True and the port is 9424 therefore.
      defaultSsl = (ssl is None and len(root_cert) == 0) or (ssl is True or len(root_cert) != 0)
      (h, p) = self._get_host_port(host, defaultSsl)
      self.host = h + ":" + p
      self.ssl = ssl is True or len(root_cert) != 0 or p == "8524"
      self.root_cert = root_cert
      self.auth_token = auth_token
      self.auth_secret = auth_secret
      if self.ssl or self.root_cert:
        self.channel = self._create_secure_channel()
      else:
        self.channel = grpc.insecure_channel(self.host)
    
    def _get_host_port(self, host, defaultSsl):
      portRe = r"^(?P<host>[^:]+):(?P<port>[0-9]+)$"
      matches = re.search(portRe, host)
      defaultPort = defaultSsl and "8524" or "8523"
      return (host, defaultPort) if matches is None else (matches.group("host"), matches.group("port"))
    
    def _metadata_callback(self, context, callback):
      callback([('token', self.auth_token), ('secret', self.auth_secret)], None)

    def _create_secure_channel(self):
        if len(self.root_cert) != 0:
          cert_creds = grpc.ssl_channel_credentials(root_certificates=self.root_cert)
        else:
          cert_creds = grpc.ssl_channel_credentials()
        auth_creds = grpc.metadata_call_credentials(self._metadata_callback)
        combined_creds = grpc.composite_channel_credentials(cert_creds, auth_creds)
        channel = grpc.secure_channel(target=self.host, credentials=combined_creds)
        return channel

    def list_functions(self, request=FunctionRequest()) -> Iterable[Function]:
        """
        Lists the available functions.

        Args:
          request (FunctionRequest): The request to send. Defaults to an empty request.
        
        Returns:
          Iterable[Function]: The functions available.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetFunctions(request)
  
    def list_projects(self, request=GetProjectsRequest()) -> Iterable[Project]:
        """
        Lists the available projects.

        Args:
          request (GetProjectsRequest): The request to send. Defaults to an empty request.
        
        Returns:
          Iterable[Project]: The projects available.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetProjects(request)
    
    def list_intents(self, request: GetIntentsRequest) -> Iterable[Intent]:
        """
        Lists the available intents for a given project.

        Args:
          request (GetIntentsRequest): The request to send.
        
        Returns:
          Iterable[GetIntentsResponse]: The intents available.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetIntents(request)
    
    def list_embedding_models(self, request=GetEmbeddingModelsRequest()) -> Iterable[EmbeddingModel]:
      """
      Lists the available embedding models.

      Args:
        request (GetEmbeddingModelsRequest): The request to send. Defaults to an empty request.

      Returns:
        Iterable[EmbeddingModel]: The embedding models available.
      """
      stub = NLPServerStub(self.channel)
      return stub.GetEmbeddingModels(request)
  
    def run_functions(self, request: RunFunctionsRequest) -> RunFunctionsResponse:
        """
        Processes a raw text.

        Args:
          request (RunFunctionsRequest): The request to send.
        
        Returns:
          Iterable[RunFunctionsResponse]: The responses with the processed text.
        """
        stub = NLPServerStub(self.channel)
        return stub.RunFunctions(request)
    
    def score_limits(self, request: GetScoreLimitsRequest) -> GetScoreLimitsResponse:
        """
        Gets the score limits for a given project.

        Args:
          request (GetScoreLimitsRequest): The request to send.
        
        Returns:
          GetScoreLimitsResponse: The score limits.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetScoreLimits(request)
    
    def get_content(self, request: GetContentRequest) -> Iterable[GetContentResponse]:
        """
        Gets the content for a given project.

        Args:
          request (GetContentRequest): The request to send.
        
        Returns:
          Iterable[GetContentResponse]: The content available.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetContent(request)
    
    def update_content(self, request: UpdateContentRequest) -> UpdateContentResponse:
        """
        Updates the content that matches the given request.

        Args:
          request (UpdateContentRequest): The request to send.
        
        Returns:
          UpdateContentResponse: The response.
        """
        stub = NLPServerStub(self.channel)
        return stub.UpdateContent(request)
    
    def remove_content(self, request: RemoveContentRequest) -> RemoveContentResponse:
        """
        Removes the content that matches the given request.

        Args:
          request (RemoveContentRequest): The request to send.
        
        Returns:
          RemoveContentResponse: The response.
        """
        stub = NLPServerStub(self.channel)
        return stub.RemoveContent(request)
    
    def update_project(self, request: UpdateProjectRequest) -> UpdateProjectResponse:
        """
        Updates the project that matches the given request.

        Args:
          request (UpdateContentRequest): The request to send.
        
        Returns:
          UpdateProjectResponse: The response.
        """
        stub = NLPServerStub(self.channel)
        return stub.UpdateProject(request)
    
    def remove_project(self, request: RemoveProjectRequest) -> RemoveProjectResponse:
        """
        Removes the project that matches the given request.

        Args:
          request (RemoveProjectRequest): The request to send.
        
        Returns:
          RemoveProjectResponse: The response.
        """
        stub = NLPServerStub(self.channel)
        return stub.RemoveProject(request)
    
    def add_projects(self, request: AddProjectRequest) -> AddProjectResponse:
        """
        Adds a project with the given request.

        Args:
          request (AddProjectRequest): The request to send.
        
        Returns:
          AddProjectReponse: The response.
        """
        stub = NLPServerStub(self.channel)
        return stub.AddProject(request)

    def add_documents(self, request: AddDocumentsRequest) -> AddDocumentsResponse:
        """
        Adds documents to a project.

        The `data` field of each document must contain the raw file content.
        If `auto_process` is set, the server splits the documents into intents
        right away; otherwise call `process_documents` later.

        Args:
          request (AddDocumentsRequest): The request to send.

        Returns:
          AddDocumentsResponse: One result per document plus the number of
          succeeded and failed documents.
        """
        stub = NLPServerStub(self.channel)
        return stub.AddDocuments(request)

    def get_documents(self, request: GetDocumentsRequest) -> GetDocumentsResponse:
        """
        Gets the documents of a project.

        The raw document content is only included when `include_data` is set.
        Use `limit` and `offset` for pagination; `total_count` in the response
        holds the number of all matching documents.

        Args:
          request (GetDocumentsRequest): The request to send.

        Returns:
          GetDocumentsResponse: The documents and the total count.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetDocuments(request)

    def process_documents(self, request: ProcessDocumentsRequest) -> ProcessDocumentsResponse:
        """
        Processes documents of a project into intents.

        Args:
          request (ProcessDocumentsRequest): The request to send.

        Returns:
          ProcessDocumentsResponse: One result per document with the created intents.
        """
        stub = NLPServerStub(self.channel)
        return stub.ProcessDocuments(request)

    def update_documents(self, request: UpdateDocumentsRequest) -> UpdateDocumentsResponse:
        """
        Updates documents of a project.

        The `id` of each document is required. Only fill `data` if the
        document content should be replaced.

        Args:
          request (UpdateDocumentsRequest): The request to send.

        Returns:
          UpdateDocumentsResponse: One result per document plus the number of
          succeeded and failed documents.
        """
        stub = NLPServerStub(self.channel)
        return stub.UpdateDocuments(request)

    def remove_documents(self, request: RemoveDocumentsRequest) -> RemoveDocumentsResponse:
        """
        Removes documents from a project.

        Set `cascade_delete` to also remove the intents that were created
        from the documents.

        Args:
          request (RemoveDocumentsRequest): The request to send.

        Returns:
          RemoveDocumentsResponse: One result per document plus the number of
          succeeded and failed documents.
        """
        stub = NLPServerStub(self.channel)
        return stub.RemoveDocuments(request)

    def get_document_status(self, request: GetDocumentStatusRequest) -> GetDocumentStatusResponse:
        """
        Gets the processing status of documents in a project.

        Args:
          request (GetDocumentStatusRequest): The request to send.

        Returns:
          GetDocumentStatusResponse: The processing status per document.
        """
        stub = NLPServerStub(self.channel)
        return stub.GetDocumentStatus(request)

    def close(self):
      if hasattr(self, "channel"):
          self.channel.close()