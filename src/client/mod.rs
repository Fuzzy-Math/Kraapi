use hyper::{Body, Client, Request};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::header::{CONTENT_TYPE, USER_AGENT};
use hyper_tls::HttpsConnector;

use super::auth::KrakenAuth;
use crate::api::{KrakenInput, MethodType};
use crate::api;

type HttpClient = Box<hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>>;

pub struct KrakenClient {
    url: String,
    version: String,
    auth: KrakenAuth,
    client: HttpClient,
}

impl KrakenClient {
    pub fn new(key: &str, secret: &str) -> Self {
        let https = HttpsConnector::new();
        KrakenClient {
            url: String::from("https://api.kraken.com"),
            version: String::from("0"),
            auth: KrakenAuth::new(&key, &secret),
            client: Box::new(Client::builder()
                .pool_idle_timeout(None)
                .http1_title_case_headers(true)
                .build::<_, hyper::Body>(https))
        }
    }

    pub fn url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub fn version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    pub fn auth(&mut self, key: &str, secret: &str) {
        self.auth = KrakenAuth::new(&key, &secret);
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_auth(&self) -> &KrakenAuth {
        &self.auth
    }

    /*
    pub fn request(&self, request: hyper::Request<hyper::Body>) -> ResponseFuture {
        self.client.request(request)
    }
    */
    pub fn request(&self, input: &KrakenInput) -> ResponseFuture {
        match input.get_info().get_type() {
            MethodType::PUBLIC => {
                let endpoint = format!("/{}/{}/{}", self.get_version(), 
                    input.get_info().get_type().to_string(), input.get_info().get_endpoint());
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
                
                request.headers_mut().insert(USER_AGENT, "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap());
                request.headers_mut().insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

                println!("{:?}", request);
                self.client.request(request)        
            },
            MethodType::PRIVATE => {
                let endpoint = format!("/{}/{}/{}", self.get_version(), 
                    input.get_info().get_type().to_string(), input.get_info().get_endpoint());
                let params = input.get_params();
                let formatted_params = api::format_params(&params).unwrap();
                // FIXME: Clean up the details behind get_params(), format_params() and KrakenInput
                // It seems to work but the references are fragile
                let signature = self.get_auth().sign(&endpoint,
                    &params.expect("Add nonce when building private methods")
                    .get("nonce").expect("Add nonce when building private methods"), 
                    &formatted_params);
                let full_url = format!("{}{}", self.get_url(), endpoint);

                let mut request = Request::builder()
                    .method("POST")
                    .uri(full_url)
                    .body(Body::from(formatted_params))
                    .expect("Failed to form a correct http request");

                request.headers_mut().insert(USER_AGENT, "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap());
                request.headers_mut().insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
                request.headers_mut().insert("API-Key", self.get_auth().get_key().parse().unwrap());
                request.headers_mut().insert("API-Sign", signature.parse().unwrap());

                println!("{:?}", request);
                self.client.request(request)
            },
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
        assert_eq!((client.auth.get_key().to_owned(), client.auth.get_secret().to_owned()), (String::from("key"), String::from("secret")));

        client.url("https://new.url.com");
        client.version("2");
        client.auth("newkey", "newsecret");

        assert_eq!(client.url, "https://new.url.com");
        assert_eq!(client.version, "2");
        assert_eq!((client.auth.get_key().to_owned(), client.auth.get_secret().to_owned()), (String::from("newkey"), String::from("newsecret")));
    }
}
