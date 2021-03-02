//! Asynchronous HTTP client implementation sending instances of [KrakenInput] to the Kraken servers
use hyper::body;
use hyper::client::HttpConnector;
use hyper::header::{CONTENT_TYPE, USER_AGENT};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

use super::auth::KrakenAuth;
use crate::api;
use crate::api::{KrakenInput, KrakenResult, MethodType};

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
    pub fn url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    /// Set the API version number as defined by Kraken
    ///
    /// Defaults to `0`
    pub fn version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    /// Assign new credentials for this KrakenClient
    pub fn auth(&mut self, key: &str, secret: &str) {
        self.auth = KrakenAuth::new(&key, &secret);
    }

    /// Returns the current base url that this client will send requests to
    pub fn get_url(&self) -> &String {
        &self.url
    }

    /// Returns the current API version that this client is using
    pub fn get_version(&self) -> &String {
        &self.version
    }

    fn get_auth(&self) -> &KrakenAuth {
        &self.auth
    }

    /// Make a request to the desired API endpoint by passing a fully constructed [KrakenInput]
    ///
    /// ## Note
    ///
    /// The types of the input and the output must match otherwise the parsing will fail
    ///
    /// For instance: if `input` is constructed from a KITicker instance, then `T` must be KOTicker
    pub async fn request<'a, T>(
        &self,
        input: &KrakenInput,
    ) -> Result<KrakenResult<T>, Box<dyn std::error::Error>>
    where
        KrakenResult<T>: DeserializeOwned,
    {
        match input.get_info().get_type() {
            MethodType::Public => {
                let endpoint = format!(
                    "/{}/{}/{}",
                    self.get_version(),
                    input.get_info().get_type().to_string(),
                    input.get_info().get_endpoint()
                );
                let formatted_params = api::format_params(&input.get_params());
                let full_url = match formatted_params {
                    Some(params) => format!("{}{}?{}", self.get_url(), endpoint, &params),
                    None => format!("{}{}", self.get_url(), endpoint),
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

                Ok(serde_json::from_slice(
                    &body::to_bytes(self.client.request(request).await?).await?,
                )?)
            }
            MethodType::Private => {
                let endpoint = format!(
                    "/{}/{}/{}",
                    self.get_version(),
                    input.get_info().get_type().to_string(),
                    input.get_info().get_endpoint()
                );
                let params = input.get_params();
                let formatted_params = api::format_params(&params).unwrap();
                // FIXME: Clean up the details behind get_params(), format_params() and KrakenInput
                // It seems to work but the references are fragile
                let signature = self.get_auth().sign(
                    &endpoint,
                    &params
                        .expect("Add nonce when building private methods")
                        .get("nonce")
                        .expect("Add nonce when building private methods"),
                    &formatted_params,
                );
                let full_url = format!("{}{}", self.get_url(), endpoint);

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
                    .insert("API-Key", self.get_auth().get_key().parse().unwrap());
                request
                    .headers_mut()
                    .insert("API-Sign", signature.parse().unwrap());

                Ok(serde_json::from_slice(
                    &body::to_bytes(self.client.request(request).await?).await?,
                )?)
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
                client.auth.get_key().to_owned(),
                client.auth.get_secret().to_owned()
            ),
            (String::from("key"), String::from("secret"))
        );

        client.url("https://new.url.com");
        client.version("2");
        client.auth("newkey", "newsecret");

        assert_eq!(client.url, "https://new.url.com");
        assert_eq!(client.version, "2");
        assert_eq!(
            (
                client.auth.get_key().to_owned(),
                client.auth.get_secret().to_owned()
            ),
            (String::from("newkey"), String::from("newsecret"))
        );
    }
}
