mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, get_keywords, nlp_service::GetKeywordsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let keywords = get_keywords(
        &mut client,
        GetKeywordsRequest {
            project_id: std::env::var("PROJECT_ID")?,
        },
    )
    .await?;

    for keyword in keywords {
        println!("{:#?}", keyword);
    }
    Ok(())
}
