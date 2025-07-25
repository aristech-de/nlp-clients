//! # Aristech NLP-Client
//! This is a client library for the Aristech NLP-Server.

#![warn(missing_docs)]

/// The nlp_server module contains types and functions generated from the Aristech NLP proto file.
pub mod nlp_service {
    #![allow(missing_docs)]
    tonic::include_proto!("aristech.nlp");
}

use std::error::Error;

use nlp_service::nlp_server_client::NlpServerClient;
use nlp_service::{
    Function, FunctionRequest, GetContentRequest, GetContentResponse, GetIntentsRequest,
    GetProjectsRequest, GetScoreLimitsRequest, GetScoreLimitsResponse, Project,
    RemoveContentRequest, RemoveContentResponse, RunFunctionsRequest, RunFunctionsResponse,
    UpdateContentRequest, UpdateContentResponse,
};
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use crate::nlp_service::Intent;

/// The Auth struct holds the token and secret needed to authenticate with the server.
#[derive(Clone, Debug)]
pub struct Auth {
    /// The token to authenticate with the server
    pub token: String,
    /// The secret to authenticate with the server
    pub secret: String,
}

impl Auth {
    /// Creates a new Auth struct with the given token and secret.
    pub fn new(token: &str, secret: &str) -> Self {
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
        }
    }
}

/// The AuthInterceptor struct is used to intercept requests to the server and add the authentication headers.
pub struct AuthInterceptor {
    /// The authentication data to add to the headers
    auth: Option<Auth>,
}

impl AuthInterceptor {
    /// Creates a new AuthInterceptor with the given authentication data.
    fn new(auth: Option<Auth>) -> Self {
        Self { auth }
    }
}
impl Interceptor for AuthInterceptor {
    /// Adds the authentication data to the headers of the request.
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        if let Some(auth) = &self.auth {
            let mut request = request;
            request
                .metadata_mut()
                .insert("token", auth.token.parse().unwrap());
            request
                .metadata_mut()
                .insert("secret", auth.secret.parse().unwrap());
            Ok(request)
        } else {
            Ok(request)
        }
    }
}

/// The NlpClient type is a type alias for the NlpServiceClient with the AuthInterceptor.
pub type NlpClient = NlpServerClient<InterceptedService<Channel, AuthInterceptor>>;

/// The TlsOptions struct holds the tls options needed to communicate with the server.
#[derive(Clone, Default, Debug)]
pub struct TlsOptions {
    /// The authentication data to authenticate with the server
    pub auth: Option<Auth>,
    /// The root certificate to verify the server's certificate
    /// This is usually only needed when the server uses a self-signed certificate
    pub ca_certificate: Option<String>,
}

impl TlsOptions {
    /// Creates a new TlsOptions struct with the given authentication data and root certificate.
    pub fn new(auth: Option<Auth>, ca_certificate: Option<String>) -> Self {
        Self {
            auth,
            ca_certificate,
        }
    }
}

/// Creates a new [NlpClient] to communicate with the server.
///
/// # Arguments
/// * `host` - The host to connect to (might include the port number e.g. "https://nlp.example.com:8524"). Note that the protocol must be included in the host.
/// * `tls_options` - The tls options to use when connecting to the server. If None is given, the connection will be unencrypted and unauthenticated (the server must also be configured to communicate without encryption in this case).
pub async fn get_client(
    host: String,
    tls_options: Option<TlsOptions>,
) -> Result<NlpClient, Box<dyn Error>> {
    // Check if a schema is included in the host
    // otherwise add http if no tls options are given and https otherwise
    let host = if host.starts_with("http://") || host.starts_with("https://") {
        host
    } else {
        match tls_options {
            Some(_) => format!("https://{}", host),
            None => format!("http://{}", host),
        }
    };
    match tls_options {
        Some(tls_options) => {
            let tls = match tls_options.ca_certificate {
                Some(ca_certificate) => {
                    let ca_certificate = Certificate::from_pem(ca_certificate);
                    ClientTlsConfig::new().ca_certificate(ca_certificate)
                }
                None => ClientTlsConfig::with_native_roots(ClientTlsConfig::new()),
            };
            let channel = Channel::from_shared(host)?
                .tls_config(tls)?
                .connect()
                .await?;
            let client: NlpServerClient<InterceptedService<Channel, AuthInterceptor>> =
                NlpServerClient::with_interceptor(channel, AuthInterceptor::new(tls_options.auth));
            Ok(client)
        }
        None => {
            let channel = Channel::from_shared(host)?.connect().await?;
            let client: NlpServerClient<InterceptedService<Channel, AuthInterceptor>> =
                NlpServerClient::with_interceptor(channel, AuthInterceptor::new(None));
            Ok(client)
        }
    }
}

/// Gets the functions available on the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.  
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, list_functions, TlsOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let functions = list_functions(&mut client, None).await?;
///     for function in functions {
///         println!("{:?}", function);
///     }
///     Ok(())
/// }
/// ```
pub async fn list_functions(
    client: &mut NlpClient,
    request: Option<FunctionRequest>,
) -> Result<Vec<Function>, Box<dyn Error>> {
    let req = request.unwrap_or_default();
    let response = client.get_functions(req).await?;
    let mut stream = response.into_inner();
    let mut functions = vec![];
    while let Some(function) = stream.message().await? {
        functions.push(function);
    }
    Ok(functions)
}

/// Processes the given text with the specified function pipeline.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, process, TlsOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let request = ProcessRawRequest {
///         input: "Hello, world!".to_string(),
///         functions: vec![
///             Function { id: "spellcheck-de".to_string(), ..Function::default() },
///         ],
///        ..ProcessRawRequest::default()
///     };
///     let response = run_functions(&mut client, request).await?;
///     println!("{}", response.output);
///     Ok(())
/// }
pub async fn run_functions(
    client: &mut NlpClient,
    request: RunFunctionsRequest,
) -> Result<RunFunctionsResponse, Box<dyn Error>> {
    let response = client.run_functions(request).await?;
    Ok(response.into_inner())
}

/// Gets the projects available on the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, list_projects, TlsOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let projects = list_projects(&mut client, None).await?;
///     for project in projects {
///         println!("{:?}", project);
///     }
///     Ok(())
/// }
/// ```
pub async fn list_projects(
    client: &mut NlpClient,
    request: Option<GetProjectsRequest>,
) -> Result<Vec<Project>, Box<dyn Error>> {
    let req = request.unwrap_or_default();
    let response = client.get_projects(req).await?;
    let mut stream = response.into_inner();
    let mut projects = vec![];
    while let Some(project) = stream.message().await? {
        projects.push(project);
    }
    Ok(projects)
}

/// Gets the intents available on the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, get_intents, TlsOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let intents = get_intents(&mut client, None).await?;
///     for intent in intents {
///         println!("{:?}", intent);
///     }
///     Ok(())
/// }
/// ```
pub async fn get_intents(
    client: &mut NlpClient,
    request: GetIntentsRequest,
) -> Result<Vec<Intent>, Box<dyn Error>> {
    let response = client.get_intents(request).await?;
    let mut stream = response.into_inner();
    let mut intents = vec![];
    while let Some(intent) = stream.message().await? {
        intents.push(intent);
    }
    Ok(intents)
}

/// Gets the score limits for the given input.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, get_score_limits, TlsOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let request = GetScoreLimitsRequest {
///         input: "Hello, world!".to_string(),
///         ..GetScoreLimitsRequest::default()
///     };
///     let response = get_score_limits(&mut client, request).await?;
///     println!("{:?}", response);
///     Ok(())
/// }
/// ```
pub async fn get_score_limits(
    client: &mut NlpClient,
    request: GetScoreLimitsRequest,
) -> Result<GetScoreLimitsResponse, Box<dyn Error>> {
    let response = client.get_score_limits(request).await?;
    Ok(response.into_inner())
}

/// Gets the content for the given request.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, get_content, TlsOptions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let request = GetContentRequest {
///         prompt: "What are the lottery numbers?".to_string(),
///         threshold: 0.5,
///         return_payload: true,
///         num_results: 3,
///         metadata: Some(ContentMetaData {
///             project_id: "3f6959e6-cfb5-4eed-8195-033d47b73263".to_string(),
///             exclude_output_from_search: true,
///             ..ContentMetaData::default()
///         }),
///         ..GetContentRequest::default()
///     };
///     let responses = get_content(&mut client, request).await?;
///     for response in responses {
///         println!("{:?}", response.items);
///     }
///     Ok(())
/// }
/// ```
pub async fn get_content(
    client: &mut NlpClient,
    request: GetContentRequest,
) -> Result<Vec<GetContentResponse>, Box<dyn Error>> {
    let response = client.get_content(request).await?;
    let mut stream = response.into_inner();
    let mut content = vec![];
    while let Some(c) = stream.message().await? {
        content.push(c);
    }
    Ok(content)
}

/// Updates the content for the given request.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, update_content, TlsOptions, UpdateContentRequest, ContentMetaData, ContentData, DescriptionMapping, Output, OutputType};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let request = UpdateContentRequest {
///         id: "123".to_string(),
///         metadata: Some(ContentMetaData {
///             project_id: "456".to_string(),
///             ..ContentMetaData::default()
///         }),
///         content: Some(ContentData {
///             topic: "lottery".to_string(),
///             description_mappings: vec![DescriptionMapping {
///                 uuid: "82a2133e-038e-4c83-aa26-de47ed386c55".to_string(),
///                 description: "What are the lottery numbers?".to_string(),
///                 ..DescriptionMapping::default()
///             }],
///             output: vec![Output {
///                 r#type: OutputType::Chat.into(),
///                 data: "The latest lottery numbers are {{lotto_numbers}}".into(),
///             }],
///             ..ContentData::default()
///         }),
///     };
///     let response = update_content(&mut client, request).await?;
///     println!("{:?}", response);
///     Ok(())
/// }
/// ```
pub async fn update_content(
    client: &mut NlpClient,
    request: UpdateContentRequest,
) -> Result<UpdateContentResponse, Box<dyn Error>> {
    let response = client.update_content(request).await?;
    Ok(response.into_inner())
}

/// Removes the content for the given request.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_nlp_client::{get_client, remove_content, TlsOptions, RemoveContentRequest, ContentMetaData};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://nlp.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let request = RemoveContentRequest {
///         id: "123".to_string(),
///         metadata: Some(ContentMetaData {
///             project_id: "456".to_string(),
///             ..ContentMetaData::default()
///         }),
///         ..RemoveContentRequest::default()
///     };
///     let response = remove_content(&mut client, request).await?;
///     println!("{:?}", response);
///     Ok(())
/// }
/// ```
pub async fn remove_content(
    client: &mut NlpClient,
    request: RemoveContentRequest,
) -> Result<RemoveContentResponse, Box<dyn Error>> {
    let response = client.remove_content(request).await?;
    Ok(response.into_inner())
}
