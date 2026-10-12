mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, nlp_service::UpdateProjectRequest, update_project};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = update_project(
        &mut client,
        UpdateProjectRequest {
            project_id: std::env::var("PROJECT_ID")?,
            name: "Updated Example Project".to_string(),
            description: "Project updated by the Rust example.".to_string(),
            ..UpdateProjectRequest::default()
        },
    )
    .await?;

    println!("{:#?}", response);
    Ok(())
}
