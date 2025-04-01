mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, list_projects};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let projects = list_projects(&mut client, None).await?;
    for project in projects {
        println!("{:?}", project);
    }
    Ok(())
}
