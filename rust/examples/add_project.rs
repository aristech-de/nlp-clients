mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{add_project, get_client, nlp_service::AddProjectRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = add_project(
        &mut client,
        AddProjectRequest {
            name: "Example Project".to_string(),
            description: "Project created by the Rust example.".to_string(),
            ..AddProjectRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
