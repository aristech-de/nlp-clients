mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, get_score_limits, nlp_service::GetScoreLimitsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = get_score_limits(
        &mut client,
        GetScoreLimitsRequest {
            project_id: std::env::var("PROJECT_ID")?,
            test_sentences_upper_limit: vec![
                "What are the lottery numbers?".to_string(),
                "Can you tell me the winning numbers?".to_string(),
            ],
            test_sentences_lower_limit: vec![
                "What's the weather like?".to_string(),
                "I want to speak to a human.".to_string(),
            ],
        },
    )
    .await?;
    println!("{:?}", response);
    Ok(())
}
