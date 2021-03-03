//! Asynchronous HTTP client implementation sending instances of [KrakenInput] to the Kraken servers
use hyper::body;
use hyper::client::HttpConnector;
use hyper::header::{CONTENT_TYPE, USER_AGENT};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

use super::auth::KrakenAuth;
use super::error::{self, KrakenError};
use crate::api;
use crate::api::{KResult, KrakenInput, KrakenResult, MethodType};

type HttpClient = Box<hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>>;

/// Asynchronous HTTP client implementation sending instances of [KrakenInput] to the Kraken servers
pub struct KrakenClient {
    url: String,
    version: String,
    auth: KrakenAuth,
    client: HttpClient,
}

impl KrakenClient {
    /// Construct a new KrakenClient instance
    ///
    /// [KrakenInput] instances will be passed into the client and the fully parsed data will be
    /// returned
    ///
    /// ## Note
    ///
    /// If only calling public endpoints, passing empty string literals for key and secret is
    /// acceptable. However, trying to call a private endpoint with empty credentials will panic.
    /// If needing to call both public and private endpoints, a single authenticated client will
    /// suffice but unique clients can be used as well
    pub fn new(key: &str, secret: &str) -> Self {
        let https = HttpsConnector::new();
        KrakenClient {
            url: String::from("https://api.kraken.com"),
            version: String::from("0"),
            auth: KrakenAuth::new(&key, &secret),
            client: Box::new(
                Client::builder()
                    .pool_idle_timeout(None)
                    .http1_title_case_headers(true)
                    .build::<_, hyper::Body>(https),
            ),
        }
    }

    /// Set the base url where requests will be sent. Not currently useful as Kraken only has one
    /// REST API
    ///
    /// Defaults to `https://api.kraken.com`
    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    /// Set the API version number as defined by Kraken
    ///
    /// Defaults to `0`
    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    /// Assign new credentials for this KrakenClient
    pub fn set_auth(&mut self, key: &str, secret: &str) {
        self.auth = KrakenAuth::new(&key, &secret);
    }

    /// Returns the current base url that this client will send requests to
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Returns the current API version that this client is using
    pub fn version(&self) -> &String {
        &self.version
    }

    fn auth(&self) -> &KrakenAuth {
        &self.auth
    }

    /// Make a request to the desired API endpoint by passing a fully constructed [KrakenInput]
    ///
    /// ## Note
    ///
    /// The types of the input and the output must match otherwise the parsing will fail
    ///
    /// For instance: if `input` is constructed from a KITicker instance, then `T` must be KOTicker
    pub async fn request<'a, T>(&self, input: &KrakenInput) -> KrakenResult<T>
    where
        T: DeserializeOwned,
    {
        match input.info().method() {
            MethodType::Public => {
                let endpoint = format!(
                    "/{}/{}/{}",
                    self.version(),
                    input.info().method().to_string(),
                    input.info().endpoint()
                );
                let formatted_params = api::format_params(&input.params());
                let full_url = match formatted_params {
                    Some(params) => format!("{}{}?{}", self.url(), endpoint, &params),
                    None => format!("{}{}", self.url(), endpoint),
                };

                let mut request = Request::builder()
                    .method("GET")
                    .uri(full_url)
                    .body(Body::empty())
                    .expect("Failed to form a correct http request");

                request.headers_mut().insert(
                    USER_AGENT,
                    "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap(),
                );
                request.headers_mut().insert(
                    CONTENT_TYPE,
                    "application/x-www-form-urlencoded".parse().unwrap(),
                );

                /*
                let parsed: KResult<T> = serde_json::from_slice(
                    &body::to_bytes(
                        self.client.request(request).await?
                    ).await?,
                )?;
                */

                let res = match self.client.request(request).await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(Vec::<KrakenError>::from(KrakenError::from(err))),
                }?;

                let bytes = match body::to_bytes(res).await {
                    Ok(bytes) => Ok(bytes),
                    Err(err) => Err(Vec::<KrakenError>::from(KrakenError::from(err))),
                }?;

                let parsed: KResult<T> = match serde_json::from_slice(&bytes) {
                    Ok(parsed) => Ok(parsed),
                    Err(err) => Err(Vec::<KrakenError>::from(KrakenError::from(err))),
                }?;

                let api_errors = parsed.error;
                match api_errors.len() {
                    0 => Ok(parsed.result.unwrap()),
                    _ => Err(error::generate_errors(api_errors)),
                }
            }

            MethodType::Private => {
                let endpoint = format!(
                    "/{}/{}/{}",
                    self.version(),
                    input.info().method().to_string(),
                    input.info().endpoint()
                );
                let params = input.params();
                let formatted_params = api::format_params(&params).unwrap();
                // FIXME: Clean up the details behind get_params(), format_params() and KrakenInput
                // It seems to work but the references are fragile
                let signature = self.auth().sign(
                    &endpoint,
                    &params
                        .expect("Add nonce when building private methods")
                        .get("nonce")
                        .expect("Add nonce when building private methods"),
                    &formatted_params,
                );
                let full_url = format!("{}{}", self.url(), endpoint);

                let mut request = Request::builder()
                    .method("POST")
                    .uri(full_url)
                    .body(Body::from(formatted_params))
                    .expect("Failed to form a correct http request");

                request.headers_mut().insert(
                    USER_AGENT,
                    "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap(),
                );
                request.headers_mut().insert(
                    CONTENT_TYPE,
                    "application/x-www-form-urlencoded".parse().unwrap(),
                );
                request
                    .headers_mut()
                    .insert("API-Key", self.auth().key().parse().unwrap());
                request
                    .headers_mut()
                    .insert("API-Sign", signature.parse().unwrap());

                /*
                let parsed: KResult<T> = serde_json::from_slice(
                    &body::to_bytes(
                        self.client.request(request).await?
                    ).await?,
                )?;
                */

                let res = match self.client.request(request).await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(Vec::<KrakenError>::from(KrakenError::from(err))),
                }?;

                let bytes = match body::to_bytes(res).await {
                    Ok(bytes) => Ok(bytes),
                    Err(err) => Err(Vec::<KrakenError>::from(KrakenError::from(err))),
                }?;

                let parsed: KResult<T> = match serde_json::from_slice(&bytes) {
                    Ok(parsed) => Ok(parsed),
                    Err(err) => Err(Vec::<KrakenError>::from(KrakenError::from(err))),
                }?;

                let api_errors = parsed.error;
                match api_errors.len() {
                    0 => Ok(parsed.result.unwrap()),
                    _ => Err(error::generate_errors(api_errors)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_client() {
        let mut client = KrakenClient::new("key", "secret");

        assert_eq!(client.url, "https://api.kraken.com");
        assert_eq!(client.version, "0");
        assert_eq!(
            (
                client.auth.key().to_owned(),
                client.auth.secret().to_owned()
            ),
            (String::from("key"), String::from("secret"))
        );

        client.set_url("https://new.url.com");
        client.set_version("2");
        client.set_auth("newkey", "newsecret");

        assert_eq!(client.url, "https://new.url.com");
        assert_eq!(client.version, "2");
        assert_eq!(
            (
                client.auth.key().to_owned(),
                client.auth.secret().to_owned()
            ),
            (String::from("newkey"), String::from("newsecret"))
        );
    }
}
