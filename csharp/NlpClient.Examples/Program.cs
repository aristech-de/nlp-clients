using Grpc.Core;
using Aristech.Nlp;
using DotNetEnv;

internal class Program
{
  private static async Task Main(string[] args)
  {
    // Load the .env file in working directory
    Env.Load();

    var client = new NlpClient(new NlpClientOptions
    {
      Host = Environment.GetEnvironmentVariable("HOST") ?? "https://localhost:8525",
      Token = Environment.GetEnvironmentVariable("TOKEN") ?? "",
      Secret = Environment.GetEnvironmentVariable("SECRET") ?? "",
    });

    // The first argument is the method name
    if (args.Length > 0 && args[0] == "GetContent")
      await GetContent(client);
    else if (args.Length > 0 && args[0] == "GetFunctions")
      await GetFunctions(client);
    else if (args.Length > 0 && args[0] == "GetIntents")
      await GetIntents(client);
    else if (args.Length > 0 && args[0] == "Process")
      await Process(client);
    else if (args.Length > 0 && args[0] == "Projects")
      await Projects(client);
    else if (args.Length > 0 && args[0] == "Documents")
      await Documents(client, args.Length > 1 ? args[1] : null);
    else
      System.Console.WriteLine("Usage: dotnet run GetContent|GetFunctions|GetIntents|Process|Projects|Documents [file]");
  }

  // GetContent example
  private static async Task GetContent(NlpClient client)
  {
    var request = new GetContentRequest
    {
      Prompt = "Hello, how are you?",
      ProjectId = Environment.GetEnvironmentVariable("PROJECT_ID") ?? "",
      NumResults = 3,
      Threshold = 0.5f,
      ChatId = "",
    };

    var reply = client.GetContent(request);
    // Read the stream of messages
    await foreach (var item in reply.ResponseStream.ReadAllAsync())
    {
      System.Console.WriteLine(item);
    }
  }

  // GetFunctions example
  private static async Task GetFunctions(NlpClient client)
  {
    var reply = client.Functions(new FunctionRequest());
    // Read the stream of messages
    await foreach (var item in reply.ResponseStream.ReadAllAsync())
    {
      System.Console.WriteLine(item);
    }
  }

  // ListIntents example
  private static async Task GetIntents(NlpClient client)
  {
    var reply = client.Intents(new GetIntentsRequest
    {
      ProjectId = Environment.GetEnvironmentVariable("PROJECT_ID") ?? "",
    });
    // Read the stream of messages
    await foreach (var item in reply.ResponseStream.ReadAllAsync())
    {
      System.Console.WriteLine(item);
    }
  }

  // Process example
  private static async Task Process(NlpClient client)
  {
    var request = new RunFunctionsRequest
    {
      Input = "hello world",
    };
    request.Functions.Add(new Function
    {
      Id = "spellcheck-de"
    });

    var reply = await client.Process(request);
    System.Console.WriteLine(reply);
  }

  private static async Task Projects(NlpClient client)
  {
    var reply = client.Projects(new GetProjectsRequest());
    // Read the stream of messages
    await foreach (var item in reply.ResponseStream.ReadAllAsync())
    {
      System.Console.WriteLine(item);
    }
  }

  private static async Task GetScoreLimits(NlpClient client)
  {
    var request = new GetScoreLimitsRequest
    {
      ProjectId = Environment.GetEnvironmentVariable("PROJECT_ID") ?? "",
    };

    var reply = await client.GetScoreLimits(request);
    System.Console.WriteLine(reply);
  }

  // Documents example: adds the given file (if any) and lists the documents of the project
  private static async Task Documents(NlpClient client, string? filePath)
  {
    var projectId = Environment.GetEnvironmentVariable("PROJECT_ID") ?? "";

    if (filePath != null)
    {
      var contentType = Path.GetExtension(filePath).ToLowerInvariant() switch
      {
        ".pdf" => ContentType.ApplicationPdf,
        ".md" => ContentType.ApplicationMd,
        ".txt" => ContentType.ApplicationTxt,
        ".xlsx" => ContentType.ApplicationXlsx,
        ".png" => ContentType.ImagePng,
        ".jpg" or ".jpeg" => ContentType.ImageJpeg,
        ".webp" => ContentType.ImageWebp,
        _ => throw new ArgumentException($"Unsupported file type: {filePath}"),
      };
      var fileName = Path.GetFileName(filePath);
      var request = new AddDocumentsRequest
      {
        ProjectId = projectId,
        // Split the document into intents right away
        AutoProcess = true,
      };
      request.Documents.Add(new Document
      {
        Name = fileName,
        Filename = fileName,
        ContentType = contentType,
        Data = Google.Protobuf.ByteString.CopyFrom(await File.ReadAllBytesAsync(filePath)),
      });

      var added = await client.AddDocumentsAsync(request);
      foreach (var result in added.Results)
      {
        if (string.IsNullOrEmpty(result.Error))
          System.Console.WriteLine($"Added document {result.DocumentId}");
        else
          System.Console.Error.WriteLine($"Failed to add document: {result.Error}");
      }
    }

    // List the documents of the project with their processing status
    var response = await client.GetDocumentsAsync(new GetDocumentsRequest { ProjectId = projectId });
    System.Console.WriteLine($"{response.TotalCount} document(s):");
    foreach (var document in response.Documents)
    {
      var status = document.ProcessingStatus?.Status.ToString() ?? "Unknown";
      System.Console.WriteLine($"{document.Id} {document.Name} [{status}] {document.IntentIds.Count} intents");
    }
  }
}