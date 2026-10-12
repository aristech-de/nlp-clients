mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{
    get_client, list_embedding_models, nlp_service::GetEmbeddingModelsRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let models = list_embedding_models(&mut client, Some(GetEmbeddingModelsRequest {})).await?;
    for model in models {
        println!("{:#?}", model);
    }
    Ok(())
}
