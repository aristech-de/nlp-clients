use std::error::Error;

use aristech_nlp_client::{nlp_service::ContentType, Auth, TlsOptions};

/// This function checks if token and secret or a root certificate are set in the environment
/// variables and returns the TlsOptions accordingly.
pub fn get_tls_options() -> Result<Option<TlsOptions>, Box<dyn Error>> {
    // For self-signed certificates we would need to read the certificate file into a string
    // and set ca_certificate to Some(ca_certificate_string)
    let root_cert = match std::env::var("ROOT_CERT") {
        Ok(root_cert) => match root_cert.is_empty() {
            true => None,
            false => {
                let root_cert = std::fs::read_to_string(root_cert)?;
                Some(root_cert)
            }
        },
        Err(_) => None,
    };
    let tls_options = {
        match (std::env::var("TOKEN"), std::env::var("SECRET")) {
            (Ok(token), Ok(secret)) if !token.is_empty() && !secret.is_empty() => {
                Some(TlsOptions {
                    ca_certificate: root_cert,
                    auth: Some(Auth { token, secret }),
                })
            }
            _ => match root_cert {
                Some(root_cert) => Some(TlsOptions {
                    ca_certificate: Some(root_cert),
                    auth: None,
                }),
                None => match std::env::var("SSL") {
                    Ok(ssl) => match ssl.parse::<bool>() {
                        Ok(true) => Some(TlsOptions {
                            ca_certificate: None,
                            auth: None,
                        }),
                        _ => None,
                    },
                    _ => None,
                },
            },
        }
    };
    Ok(tls_options)
}

/// Maps the file extension of the given path to the corresponding content type.
#[allow(dead_code)]
pub fn content_type_from_path(path: &std::path::Path) -> Result<ContentType, Box<dyn Error>> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    match extension.as_str() {
        "png" => Ok(ContentType::ImagePng),
        "jpg" | "jpeg" => Ok(ContentType::ImageJpeg),
        "webp" => Ok(ContentType::ImageWebp),
        "md" => Ok(ContentType::ApplicationMd),
        "txt" => Ok(ContentType::ApplicationTxt),
        "pdf" => Ok(ContentType::ApplicationPdf),
        "xlsx" => Ok(ContentType::ApplicationXlsx),
        _ => Err(format!("Unsupported file type: {}", path.display()).into()),
    }
}
