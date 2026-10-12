mod utils;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use utils::{content_type_from_path, get_tls_options};

use aristech_nlp_client::{
    add_documents, get_client, get_document_status,
    nlp_service::{
        processing_status::Status, AddDocumentsRequest, Document, GetDocumentStatusRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Usage: cargo run --example document_workflow -- <file>
    let file = std::env::args()
        .nth(1)
        .ok_or("Usage: document_workflow <file>")?;
    let path = Path::new(&file);
    let content_type = content_type_from_path(path)?;
    let data = std::fs::read(path)?;
    let filename = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;
    let project_id = std::env::var("PROJECT_ID")?;
    let creator_id = std::env::var("CREATOR_ID").unwrap_or_else(|_| "rust-example".to_string());

    // Upload the document and let the server process it right away
    let response = add_documents(
        &mut client,
        AddDocumentsRequest {
            project_id: project_id.clone(),
            creator_id,
            documents: vec![Document {
                project_id: project_id.clone(),
                name: filename.clone(),
                filename,
                content_type: content_type as i32,
                data,
                ..Document::default()
            }],
            auto_process: true,
            ..AddDocumentsRequest::default()
        },
    )
    .await?;
    let result = response
        .results
        .into_iter()
        .next()
        .ok_or("The server returned no result for the document")?;
    if !result.error.is_empty() {
        return Err(format!("Adding the document failed: {}", result.error).into());
    }
    let document_id = result.document_id;
    println!("Added document {}", document_id);

    // Poll the processing status until the document is processed
    for _ in 0..150 {
        let response = get_document_status(
            &mut client,
            GetDocumentStatusRequest {
                project_id: project_id.clone(),
                document_ids: vec![document_id.clone()],
            },
        )
        .await?;
        let Some(status) = response.statuses.into_iter().next() else {
            return Err("The server returned no status for the document".into());
        };
        let processing_status = status.processing_status.unwrap_or_default();
        match processing_status.status() {
            Status::Completed => {
                println!(
                    "Document processed, {} intents created: {:?}",
                    status.intents_created, status.intent_ids
                );
                return Ok(());
            }
            Status::Failed => {
                return Err(
                    format!("Processing failed: {}", processing_status.error_message).into(),
                );
            }
            state => println!("{:?} ({}%)", state, processing_status.progress),
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Err("Timed out waiting for the document to be processed".into())
}
