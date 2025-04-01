# Aristech NLP-Client for Rust

This is the Rust client implementation for the Aristech NLP-Server.

## Installation

To use the client in your project, add it to your `Cargo.toml` or use `cargo` to add it:

```sh
cargo add aristech-nlp-client
```

## Usage

```rust
use aristech_nlp_client::{
  get_client, process, TlsOptions, Auth,
  nlp_service::{Function, ProcessRawRequest},
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = get_client(
    "https://nlp.example.com",
    Some(TlsOptions {
        ca_certificate: None,
        auth: Some(Auth { token: "your-token", secret: "your-secret" }),
    }),
    ).await?;

    let response = process(&mut client, ProcessRawRequest {
      input: "hello world".to_string(),
      functions: vec![
        Function { id: "spellcheck".to_string(), ..Function::default() },
      ],
      ..ProcessRawRequest::default()
    }).await?;
    println!("{}", response.output);
    Ok(())
}
```

There are several examples in the [examples](.) directory:

- [functions.rs](examples/models.rs): Demonstrates how to list the available functions.
- [process.rs](examples/process.rs): Demonstrates how to perform NLP processing on a text.
- [projects.rs](examples/projects.rs): Demonstrates how to list the available projects.
- [intents.rs](examples/intents.rs): Demonstrates how to list intents for a project.
- [scoreLimits.rs](examples/scoreLimits.rs): Demonstrates how to use score limits to figure out good thresholds for intents.
- [content.rs](examples/content.rs): Demonstrates how to search content for a given prompt.

You can run the examples directly using `cargo` like this:

1. Create a `.env` file in the [rust](.) directory:

```sh
HOST=nlp.example.com
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# If neither credentials nor an explicit root certificate are provided,
# you can still enable SSL by setting the SSL environment variable to true:
# SSL=true
# PROJECT_ID=your-project-id # Required for some examples
```

2. Run the examples, e.g.:

```sh
cargo run --example functions
```

## Build

To build the library, run:

```bash
cargo build
```