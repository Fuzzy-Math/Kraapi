use hyper::{Body, Client};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper_tls::HttpsConnector;

use super::auth::KrakenAuth;


type HttpClient = Box<hyper::Client<HttpsConnector<HttpConnector>>>;

pub struct KrakenClient {
    url: String,
    version: String,
    auth: KrakenAuth,
    client: HttpClient,
}

impl KrakenClient {
    pub fn new(key: &str, secret: &str) -> Self {
        KrakenClient {
            url: String::from("https://api.kraken.com"),
            version: String::from("0"),
            auth: KrakenAuth::new(&key, &secret),
            client: Box::new(Client::builder()
                .build::<_, hyper::Body>(HttpsConnector::new()))
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

    pub fn request(&self, request: hyper::Request<hyper::Body>) -> ResponseFuture {
        self.client.request(request)
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
