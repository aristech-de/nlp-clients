# Aristech NLP-Client for NodeJS

This is the NodeJS client implementation for the Aristech NLP-Server.

## Installation

```bash
npm install @aristech-org/nlp-client
```

## Usage

```typescript
import { NlpClient } from '@aristech-org/nlp-client'

const client = new NlpClient({ host: 'nlp.example.com' })
const response = await client.process({
  input: 'hello world',
  functions: [{ id: 'spellcheck-de' }]
})
console.log(response)
```

There are several examples in the `examples` directory:

- [functions.ts](examples/models.ts): Demonstrates how to list the available functions.
- [process.ts](examples/process.ts): Demonstrates how to perform NLP processing on a text.
- [projects.ts](examples/projects.ts): Demonstrates how to list the available projects.
- [intents.ts](examples/intents.ts): Demonstrates how to list intents for a project.
- [scoreLimits.ts](examples/scoreLimits.ts): Demonstrates how to use score limits to figure out good thresholds for intents.
- [content.ts](examples/content.ts): Demonstrates how to search content for a given prompt.

You can run the examples directly using `tsx` like this:

1. Create a `.env` file in the [node](.) directory:

```sh
HOST=nlp.example.com
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# SSL=true # Set to true if credentials are provided or if a ROOT_CERT is provided
# PROJECT_ID=your-project-id # Required for some examples
```

2. Run the examples, e.g.:

```sh
npx tsx examples/functions.ts
```

## Build

To rebuild the generated typescript files from the proto file, run:

```bash
npm run generate
```

To build the library, run:

```bash
npm run build
```

