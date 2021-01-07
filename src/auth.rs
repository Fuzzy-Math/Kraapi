use base64;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::{Sha256,Sha512};
use indexmap::map::IndexMap;
use std::fmt::Display;

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

    fn format_params<T, U>(params: &IndexMap<T, U>) -> String
        where T: Display,
              U: Display
    {
        let mut res = String::new();
        for index in 0..params.len() {
            let pair = params.get_index(index).unwrap();
            if index == 0 {
                res = format!("{}{}={}", res, pair.0, pair.1);
            } else {
                res = format!("{}&{}={}", res, pair.0, pair.1);
            }
        }
        
        return res;
    }

    pub fn sign<T, U>(&self, path: &str, nonce: &str, params: &IndexMap<T, U>) -> String
        where T: Display,
              U: Display
    {
        let params = Self::format_params(params);
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

    use super::*;
    use indexmap::map::IndexMap;

    #[test]
    fn test_auth() {
        let auth = KrakenAuth::new("CJbfPw4tnbf/9en/ZmpewCTKEwmmzO18LXZcHQcu7HPLWre4l8+V9I3y",
            "FRs+gtq09rR7OFtKj9BGhyOGS3u5vtY/EdiIBO9kD8NFtRX7w7LeJDSrX6cq1D8zmQmGkWFjksuhBvKOAWJohQ==");
        let api_path = String::from("/0/private/TradeBalance");
        let api_nonce = String::from("1540973848000");
        let mut params = IndexMap::new();
        params.insert("nonce".to_string(), api_nonce.clone());
        params.insert("asset".to_string(), "xbt".to_string());

        let signature = auth.sign(&api_path, &api_nonce, &params);

        assert_eq!(signature, String::from("RdQzoXRC83TPmbERpFj0XFVArq0Hfadm0eLolmXTuN2R24hzIqtAnF/f7vSfW1tGt7xQOn8bjm+Ht+X0KrMwlA=="));
    }
}
