public class NlpClientOptions
{
  public string Host { get; set; } = "";

  public string? Token { get; set; }
  public string? Secret { get; set; }

  public string? RootCertificatePath { get; set; }

  public bool Tls { get; set; } = true;
}