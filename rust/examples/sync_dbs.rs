mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{get_client, sync_dbs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    sync_dbs(&mut client).await?;
    println!("Databases synchronized.");
    Ok(())
}
