using Grpc.Net.Client;
using Aristech.Nlp;
using Grpc.Core;
using Grpc.Core.Interceptors;

/// <summary>
/// A client for interacting with the Aristech NLP service.
/// </summary>
/// <example>
/// <code>
/// var client = new NlpClient(new NlpClientOptions
/// {
///   Host = "https://nlp.example.com:8525",
///   Token = "my-token",
///   Secret = "my-secret",
///  });
///  </code>
///  </example>
public class NlpClient
{

  private readonly NLPServer.NLPServerClient _client;

  /// <summary>
  /// Initializes a new instance of the <see cref="NlpClient"/> class.
  /// </summary>
  /// <param name="options">Configuration for the client including URL and credentials.</param>
  public NlpClient(NlpClientOptions options)
  {
    // If the host doesn't start with http or https, depending on the TLS setting, add it
    if (!options.Host.StartsWith("http"))
    {
      options.Host = (options.Tls ? "https://" : "http://") + options.Host;
    }
    var channel = GrpcChannel.ForAddress(options.Host);
    if (!options.Tls)
    {
      // Authentication is not supported without TLS
      _client = new NLPServer.NLPServerClient(channel);
      return;
    }

    var handler = new HttpClientHandler();
    // If a root certificate path is set, use it
    if (!string.IsNullOrEmpty(options.RootCertificatePath))
    {
      handler.ServerCertificateCustomValidationCallback = HttpClientHandler.DangerousAcceptAnyServerCertificateValidator;
      var certBytes = System.IO.File.ReadAllBytes(options.RootCertificatePath);
      var certificate = System.Security.Cryptography.X509Certificates.X509CertificateLoader.LoadCertificate(certBytes);
      handler.ClientCertificates.Add(certificate);
    }

    channel = GrpcChannel.ForAddress(options.Host, new GrpcChannelOptions
    {
      HttpHandler = handler
    });

    // If token and secret are both set, use the AuthInterceptor
    if (options.Token != null && options.Secret != null)
    {
      var callInvoker = channel.Intercept(new AuthInterceptor(options.Token, options.Secret));
      _client = new NLPServer.NLPServerClient(callInvoker);
      return;
    }

    _client = new NLPServer.NLPServerClient(channel);
  }

  /// <summary>
  /// Calls the GetContent API to fetch content based on the provided request.
  /// </summary>
  /// <param name="request">The request containing prompt, filters, and other context.</param>
  /// <returns>An async stream of GetContentResponse items.</returns>
  public AsyncServerStreamingCall<GetContentResponse> GetContent(GetContentRequest request)
  {
    return _client.GetContent(request);
  }

  /// <summary>
  /// Returns a list of available functions.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <returns>An async stream of Function items.</returns>
  public AsyncServerStreamingCall<Function> Functions(FunctionRequest request)
  {
    return _client.GetFunctions(request);
  }

  /// <summary>
  /// Returns a list of available intents for the given project.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <returns>An async stream of Intent items.</returns>
  public AsyncServerStreamingCall<Intent> Intents(GetIntentsRequest request)
  {
    return _client.GetIntents(request);
  }

  /// <summary>
  /// Runs the specified functions with the provided input.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <returns>The response.</returns>
  public AsyncUnaryCall<RunFunctionsResponse> Process(RunFunctionsRequest request)
  {
    return _client.RunFunctionsAsync(request);
  }

  /// <summary>
  /// Returns a list of available projects.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <returns>An async stream of Project items.</returns>
  public AsyncServerStreamingCall<Project> Projects(GetProjectsRequest request)
  {
    return _client.GetProjects(request);
  }

  /// <summary>
  /// Returns the score limits for the given input.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <returns>The response.</returns>
  public AsyncUnaryCall<GetScoreLimitsResponse> GetScoreLimits(GetScoreLimitsRequest request)
  {
    return _client.GetScoreLimitsAsync(request);
  }

  /// <summary>
  /// Adds documents to a project.
  /// </summary>
  /// <remarks>
  /// The <see cref="Document.Data"/> field of each document must contain the raw file content.
  /// If <see cref="AddDocumentsRequest.AutoProcess"/> is set, the server splits the documents into intents right away;
  /// otherwise call <see cref="ProcessDocumentsAsync"/> later.
  /// </remarks>
  /// <param name="request">The request.</param>
  /// <param name="cancellationToken">A token to cancel the call.</param>
  /// <returns>One result per document plus the number of succeeded and failed documents.</returns>
  public AsyncUnaryCall<AddDocumentsResponse> AddDocumentsAsync(AddDocumentsRequest request, CancellationToken cancellationToken = default)
  {
    return _client.AddDocumentsAsync(request, cancellationToken: cancellationToken);
  }

  /// <summary>
  /// Gets the documents of a project.
  /// </summary>
  /// <remarks>
  /// The raw document content is only included when <see cref="GetDocumentsRequest.IncludeData"/> is set.
  /// Use <see cref="GetDocumentsRequest.Limit"/> and <see cref="GetDocumentsRequest.Offset"/> for pagination.
  /// </remarks>
  /// <param name="request">The request.</param>
  /// <param name="cancellationToken">A token to cancel the call.</param>
  /// <returns>The documents and the total number of matching documents.</returns>
  public AsyncUnaryCall<GetDocumentsResponse> GetDocumentsAsync(GetDocumentsRequest request, CancellationToken cancellationToken = default)
  {
    return _client.GetDocumentsAsync(request, cancellationToken: cancellationToken);
  }

  /// <summary>
  /// Processes documents of a project into intents.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <param name="cancellationToken">A token to cancel the call.</param>
  /// <returns>One result per document with the created intents.</returns>
  public AsyncUnaryCall<ProcessDocumentsResponse> ProcessDocumentsAsync(ProcessDocumentsRequest request, CancellationToken cancellationToken = default)
  {
    return _client.ProcessDocumentsAsync(request, cancellationToken: cancellationToken);
  }

  /// <summary>
  /// Updates documents of a project.
  /// </summary>
  /// <remarks>
  /// The <see cref="Document.Id"/> of each document is required.
  /// Only fill <see cref="Document.Data"/> if the document content should be replaced.
  /// </remarks>
  /// <param name="request">The request.</param>
  /// <param name="cancellationToken">A token to cancel the call.</param>
  /// <returns>One result per document plus the number of succeeded and failed documents.</returns>
  public AsyncUnaryCall<UpdateDocumentsResponse> UpdateDocumentsAsync(UpdateDocumentsRequest request, CancellationToken cancellationToken = default)
  {
    return _client.UpdateDocumentsAsync(request, cancellationToken: cancellationToken);
  }

  /// <summary>
  /// Removes documents from a project.
  /// </summary>
  /// <remarks>
  /// Set <see cref="RemoveDocumentsRequest.CascadeDelete"/> to also remove the intents that were created from the documents.
  /// </remarks>
  /// <param name="request">The request.</param>
  /// <param name="cancellationToken">A token to cancel the call.</param>
  /// <returns>One result per document plus the number of succeeded and failed documents.</returns>
  public AsyncUnaryCall<RemoveDocumentsResponse> RemoveDocumentsAsync(RemoveDocumentsRequest request, CancellationToken cancellationToken = default)
  {
    return _client.RemoveDocumentsAsync(request, cancellationToken: cancellationToken);
  }

  /// <summary>
  /// Gets the processing status of documents in a project.
  /// </summary>
  /// <param name="request">The request.</param>
  /// <param name="cancellationToken">A token to cancel the call.</param>
  /// <returns>The processing status per document.</returns>
  public AsyncUnaryCall<GetDocumentStatusResponse> GetDocumentStatusAsync(GetDocumentStatusRequest request, CancellationToken cancellationToken = default)
  {
    return _client.GetDocumentStatusAsync(request, cancellationToken: cancellationToken);
  }
}