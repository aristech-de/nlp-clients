mod utils;
use std::error::Error;
use std::path::Path;
use utils::{content_type_from_path, get_tls_options};

use aristech_nlp_client::{
    add_documents, get_client,
    nlp_service::{AddDocumentsRequest, ContentType, Document},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;
    let project_id = std::env::var("PROJECT_ID")?;
    let creator_id = std::env::var("CREATOR_ID").unwrap_or_else(|_| "rust-example".to_string());

    // Usage: cargo run --example add_documents -- [file]
    // Without a file a small text document is uploaded.
    let (filename, content_type, data) = match std::env::args().nth(1) {
        Some(file) => {
            let path = Path::new(&file);
            (
                path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default(),
                content_type_from_path(path)?,
                std::fs::read(path)?,
            )
        }
        None => (
            "example.txt".to_string(),
            ContentType::ApplicationTxt,
            b"Hello from the Rust add_documents example.".to_vec(),
        ),
    };

    let response = add_documents(
        &mut client,
        AddDocumentsRequest {
            project_id: project_id.clone(),
            creator_id,
            documents: vec![Document {
                project_id,
                name: filename.clone(),
                filename,
                description: "Document uploaded by the Rust example.".to_string(),
                content_type: content_type as i32,
                data,
                ..Document::default()
            }],
            ..AddDocumentsRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
