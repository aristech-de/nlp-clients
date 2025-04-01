using Grpc.Core;
using Grpc.Core.Interceptors;

public class AuthInterceptor : Interceptor
{
  private readonly string _token;
  private readonly string _secret;

  public AuthInterceptor(string token, string secret)
  {
    _token = token;
    _secret = secret;
  }

  public override AsyncUnaryCall<TResponse> AsyncUnaryCall<TRequest, TResponse>(
    TRequest request,
    ClientInterceptorContext<TRequest, TResponse> context,
    AsyncUnaryCallContinuation<TRequest, TResponse> continuation)
  {
    var headers = context.Options.Headers ?? new Metadata();
    headers.Add("token", _token);
    headers.Add("secret", _secret);

    var newOptions = context.Options.WithHeaders(headers);
    var newContext = new ClientInterceptorContext<TRequest, TResponse>(
        context.Method, context.Host, newOptions);

    return continuation(request, newContext);
  }

  public override AsyncServerStreamingCall<TResponse> AsyncServerStreamingCall<TRequest, TResponse>(
      TRequest request,
      ClientInterceptorContext<TRequest, TResponse> context,
      AsyncServerStreamingCallContinuation<TRequest, TResponse> continuation)
  {
    var headers = context.Options.Headers ?? new Metadata();
    headers.Add("token", _token);
    headers.Add("secret", _secret);

    var options = context.Options.WithHeaders(headers);
    var newContext = new ClientInterceptorContext<TRequest, TResponse>(context.Method, context.Host, options);
    return continuation(request, newContext);
  }
}