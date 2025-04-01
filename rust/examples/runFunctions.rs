mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_nlp_client::{
    get_client,
    nlp_service::{Function, RunFunctionsRequest},
    run_functions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let response = run_functions(
        &mut client,
        RunFunctionsRequest {
            input: "hello world".to_string(),
            functions: vec![Function {
                id: "spellcheck-de".to_string(),
                ..Function::default()
            }],
            ..RunFunctionsRequest::default()
        },
    )
    .await?;
    println!("{}", response.output);
    Ok(())
}
