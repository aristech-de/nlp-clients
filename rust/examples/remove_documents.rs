mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, nlp_service::RemoveDocumentsRequest, remove_documents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = remove_documents(
        &mut client,
        RemoveDocumentsRequest {
            project_id: std::env::var("PROJECT_ID")?,
            document_ids: vec![std::env::var("DOCUMENT_ID")?],
            ..RemoveDocumentsRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
