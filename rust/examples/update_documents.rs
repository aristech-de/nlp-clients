mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{
    get_client,
    nlp_service::{ContentType, Document, UpdateDocumentsRequest},
    update_documents,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;
    let project_id = std::env::var("PROJECT_ID")?;

    let response = update_documents(
        &mut client,
        UpdateDocumentsRequest {
            project_id: project_id.clone(),
            documents: vec![Document {
                id: std::env::var("DOCUMENT_ID")?,
                project_id,
                name: "Updated Example Document".to_string(),
                filename: "example-updated.txt".to_string(),
                description: "Document updated by the Rust example.".to_string(),
                content_type: ContentType::ApplicationTxt as i32,
                data: b"Updated content from the Rust update_documents example.".to_vec(),
                ..Document::default()
            }],
            ..UpdateDocumentsRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
