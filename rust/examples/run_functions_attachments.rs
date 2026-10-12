mod utils;
use std::error::Error;
use std::path::Path;
use utils::{content_type_from_path, get_tls_options};

use aristech_nlp_client::{
    get_client, list_functions,
    nlp_service::{Attachment, Function, RunFunctionsRequest},
    run_functions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Usage: cargo run --example run_functions_attachments -- <file> <function-id> [input]
    let mut args = std::env::args().skip(1);
    let file = args
        .next()
        .ok_or("Usage: run_functions_attachments <file> <function-id> [input]")?;
    let function_id = args
        .next()
        .ok_or("Usage: run_functions_attachments <file> <function-id> [input]")?;
    let input = args.next().unwrap_or_default();

    let path = Path::new(&file);
    let content_type = content_type_from_path(path)?;
    let data = std::fs::read(path)?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    // Make sure the function exists and accepts this kind of attachment
    let functions = list_functions(&mut client, None).await?;
    let function = functions
        .into_iter()
        .find(|f| f.id == function_id)
        .ok_or(format!(
            "Function '{}' not found on the server",
            function_id
        ))?;
    if !function.supports_attachments {
        return Err(format!("Function '{}' does not support attachments", function_id).into());
    }
    if !function.supported_content_types.is_empty()
        && !function
            .supported_content_types
            .contains(&(content_type as i32))
    {
        return Err(format!(
            "Function '{}' does not support {:?}",
            function_id, content_type
        )
        .into());
    }

    let response = run_functions(
        &mut client,
        RunFunctionsRequest {
            input,
            functions: vec![Function {
                id: function_id,
                ..Function::default()
            }],
            attachments: vec![Attachment {
                name: path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default(),
                content_type: content_type as i32,
                data,
            }],
            ..RunFunctionsRequest::default()
        },
    )
    .await?;
    println!("{}", response.output);
    Ok(())
}
