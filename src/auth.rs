use base64;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::{Sha256,Sha512};
use std::time::SystemTime;

pub struct KrakenAuth {
    api_key: String,
    api_secret: String,
}

impl KrakenAuth {
    pub fn new(key: &str, secret: &str) -> Self {
        KrakenAuth {
            api_key: key.to_string(),
            api_secret: secret.to_string()
        }
    }

    pub fn get_key(&self) -> &String {
        &self.api_key
    }

    pub fn get_secret(&self) -> &String {
        &self.api_secret
    }

    pub fn nonce() -> String {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let nonce = (duration.as_secs() * 1_000_000u64) + u64::from(duration.subsec_micros());

        (nonce as u64).to_string()
    }

    pub fn sign(&self, path: &str, nonce: &str, params: &str) -> String {
        let api_secret = base64::decode(&self.api_secret).unwrap();
        let mut sha256 = Sha256::new();
        let mut hmac = Hmac::new(Sha512::new(), &api_secret);
        let mut sha_res: [u8; 32] = [0; 32];
        

        sha256.input(nonce.as_bytes());
        sha256.input(params.as_bytes());
        sha256.result(&mut sha_res);

        hmac.input(path.as_bytes());
        hmac.input(&sha_res);

        base64::encode(hmac.result().code())
    }
}

#[cfg(test)]
mod tests {

    use indexmap::map::IndexMap;
    use super::*;
    use crate::api;

    #[test]
    fn test_auth() {
        let auth = KrakenAuth::new("CJbfPw4tnbf/9en/ZmpewCTKEwmmzO18LXZcHQcu7HPLWre4l8+V9I3y",
            "FRs+gtq09rR7OFtKj9BGhyOGS3u5vtY/EdiIBO9kD8NFtRX7w7LeJDSrX6cq1D8zmQmGkWFjksuhBvKOAWJohQ==");
        let api_path = String::from("/0/private/TradeBalance");
        let api_nonce = String::from("1540973848000");
        let mut params = IndexMap::new();
        params.insert("nonce".to_string(), api_nonce.clone());
        params.insert("asset".to_string(), "xbt".to_string());

        let signature = auth.sign(&api_path, &api_nonce, &api::private::format_params(&params));

        assert_eq!(signature, String::from("RdQzoXRC83TPmbERpFj0XFVArq0Hfadm0eLolmXTuN2R24hzIqtAnF/f7vSfW1tGt7xQOn8bjm+Ht+X0KrMwlA=="));
    }
}
