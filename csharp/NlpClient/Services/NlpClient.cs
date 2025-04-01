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
    return _client.ProcessAsync(request);
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
}