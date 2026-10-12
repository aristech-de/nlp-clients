mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, get_documents, nlp_service::GetDocumentsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = get_documents(
        &mut client,
        GetDocumentsRequest {
            project_id: std::env::var("PROJECT_ID")?,
            include_data: false,
            ..GetDocumentsRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
