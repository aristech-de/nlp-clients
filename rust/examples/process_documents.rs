mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, nlp_service::ProcessDocumentsRequest, process_documents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = process_documents(
        &mut client,
        ProcessDocumentsRequest {
            project_id: std::env::var("PROJECT_ID")?,
            document_ids: vec![std::env::var("DOCUMENT_ID")?],
            ..ProcessDocumentsRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
