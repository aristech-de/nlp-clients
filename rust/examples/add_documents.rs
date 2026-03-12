mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{
    add_documents, get_client,
    nlp_service::{AddDocumentsRequest, ContentType, Document},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;
    let project_id = std::env::var("PROJECT_ID")?;
    let creator_id = std::env::var("CREATOR_ID").unwrap_or_else(|_| "codex-example".to_string());

    let response = add_documents(
        &mut client,
        AddDocumentsRequest {
            project_id: project_id.clone(),
            creator_id,
            documents: vec![Document {
                project_id,
                name: "Example Document".to_string(),
                filename: "example.txt".to_string(),
                description: "Document uploaded by the Rust example.".to_string(),
                content_type: ContentType::ApplicationTxt as i32,
                data: b"Hello from the Rust add_documents example.".to_vec(),
                ..Document::default()
            }],
            ..AddDocumentsRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
