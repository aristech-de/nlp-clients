mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, get_content, nlp_service::GetContentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Use the first argument or a default prompt
    let prompt = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "What are the lottery numbers?".to_string());

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let responses = get_content(
        &mut client,
        GetContentRequest {
            prompt,
            // Since v3.0.0 the threshold field is optional.
            // To use the project's default threshold set it to None.
            threshold: Some(0.9),
            num_results: 3,
            project_id: std::env::var("PROJECT_ID")?,
            ..GetContentRequest::default()
        },
    )
    .await?;
    for response in responses {
        println!("{:#?}", response.items);
    }

    // Update content:
    /*
    use aristech_nlp_client::nlp_service::UpdateContentRequest;
    use aristech_nlp_client::nlp_service::{Intent, IntentInput};
    use aristech_nlp_client::update_content;

    let update_response = update_content(
        &mut client,
        UpdateContentRequest {
            intent: vec![Intent {
                id: "123".to_string(),
                project_id: std::env::var("PROJECT_ID")?,
                topic: "lottery".to_string(),
                inputs: vec![IntentInput {
                    uuid: "82a2133e-038e-4c83-aa26-de47ed386c55".to_string(),
                    input: "What are the lottery numbers?".to_string(),
                }],
                ..Intent::default()
            }],
        },
    )
    .await?;
    println!("{:?}", update_response);
    */

    // Remove content:
    /*
    use aristech_nlp_client::nlp_service::RemoveContentRequest;
    use aristech_nlp_client::remove_content;

    let remove_response = remove_content(
        &mut client,
        RemoveContentRequest {
            id: vec!["123".to_string()],
            project_id: std::env::var("PROJECT_ID")?,
            ..RemoveContentRequest::default()
        },
    )
    .await?;
    println!("{:?}", remove_response);
    */

    Ok(())
}
