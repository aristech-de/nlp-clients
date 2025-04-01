# Aristech NLP-Client for C#

This is the C# client implementation for the Aristech NLP-Server.

## Installation

The package is not published to NuGet yet but will be soon.

<!-- 
```bash
dotnet add package Aristech.NlpClient
``` -->

## Usage

```csharp
using Aristech.NlpClient;

var client = new NlpClient
{
    Host = "nlp.example.com:443",
    Token = "your-token",
    Secret = "your-secret",
};

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
```

There are several examples in the [NlpClient.Examples](NlpClient.Examples) directory which can be run using `dotnet run ExampleName`.

- `GetContent` - Demonstrates how to search content for a given prompt.
- `GetFunctions` - Demonstrates how to list the available functions.
- `GetIntents` - Demonstrates how to list intents for a project.
- `Projects` - Demonstrates how to list the available projects.
- `GetScoreLimits` - Demonstrates how to get score limits for the given input.
- `Process` - Demonstrates how to process text with a specified pipeline of functions.

