mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, list_functions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let functions = list_functions(&mut client, None).await?;
    for function in functions {
        println!("{:#?}", function);
    }
    Ok(())
}
