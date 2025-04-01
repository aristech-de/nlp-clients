mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, get_intents, nlp_service::GetIntentsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let intents = get_intents(
        &mut client,
        GetIntentsRequest {
            project_id: std::env::var("PROJECT_ID")?,
            ..GetIntentsRequest::default()
        },
    )
    .await?;
    for intent in intents {
        println!("{:?}", intent);
    }
    Ok(())
}
