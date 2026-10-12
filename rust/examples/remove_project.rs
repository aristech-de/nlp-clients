mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, nlp_service::RemoveProjectRequest, remove_project};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = remove_project(
        &mut client,
        RemoveProjectRequest {
            project_id: std::env::var("REMOVE_PROJECT_ID")?,
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
