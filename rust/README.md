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
    get_client, run_functions, Auth, TlsOptions,
    nlp_service::{Function, RunFunctionsRequest},
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = get_client(
        "https://nlp.example.com".to_string(),
        Some(TlsOptions::new(Some(Auth::new("your-token", "your-secret")), None)),
    )
    .await?;

    let response = run_functions(&mut client, RunFunctionsRequest {
        input: "hello world".to_string(),
        functions: vec![
            Function { id: "spellcheck".to_string(), ..Function::default() },
        ],
        ..RunFunctionsRequest::default()
    }).await?;
    println!("{}", response.output);
    Ok(())
}
```

There are several examples in the [examples](examples) directory:

Functions:
- [functions.rs](examples/functions.rs): Lists the available functions.
- [run_functions.rs](examples/run_functions.rs): Performs NLP processing on a text.
- [run_functions_attachments.rs](examples/run_functions_attachments.rs): Runs a function with a file attachment (e.g. an image or a PDF). Usage: `cargo run --example run_functions_attachments -- <file> <function-id> [input]`

Projects and content:
- [projects.rs](examples/projects.rs): Lists the available projects.
- [add_project.rs](examples/add_project.rs), [update_project.rs](examples/update_project.rs), [remove_project.rs](examples/remove_project.rs): Create, update and remove a project.
- [list_embedding_models.rs](examples/list_embedding_models.rs): Lists the embedding models that can be used for projects.
- [intents.rs](examples/intents.rs): Lists the intents of a project.
- [get_keywords.rs](examples/get_keywords.rs): Lists the keywords of a project.
- [score_limits.rs](examples/score_limits.rs): Uses score limits to figure out good thresholds for intents.
- [content.rs](examples/content.rs): Searches content for a given prompt.
- [sync_dbs.rs](examples/sync_dbs.rs): Triggers a database synchronization on the server.

Documents:
- [document_workflow.rs](examples/document_workflow.rs): Uploads a file, lets the server process it and waits until the intents are created. Usage: `cargo run --example document_workflow -- <file>`
- [add_documents.rs](examples/add_documents.rs): Uploads a document (optionally a file given as first argument).
- [get_documents.rs](examples/get_documents.rs), [get_document_status.rs](examples/get_document_status.rs): Lists documents and their processing status.
- [process_documents.rs](examples/process_documents.rs), [update_documents.rs](examples/update_documents.rs), [remove_documents.rs](examples/remove_documents.rs): Processes, updates and removes a document.

Supported file types for attachments and documents are `png`, `jpg`/`jpeg`, `webp`, `md`, `txt`, `pdf` and `xlsx`.

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
# DOCUMENT_ID=your-document-id # Required for the examples that work with an existing document
# CREATOR_ID=your-user-id # Used when adding documents (defaults to rust-example)
# REMOVE_PROJECT_ID=project-to-delete # Only used by remove_project, which deletes this project
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
