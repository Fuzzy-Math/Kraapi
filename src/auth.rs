use hmac::{Hmac, Mac, NewMac};
use sha2::{Sha256, Sha512, Digest};
use std::time::SystemTime;

type HmacSha512 = Hmac<Sha512>;

pub(crate) struct KrakenAuth {
    api_key: String,
    api_secret: String,
}

impl KrakenAuth {
    pub(crate) fn new(key: &str, secret: &str) -> Self {
        KrakenAuth {
            api_key: key.to_string(),
            api_secret: secret.to_string(),
        }
    }

    pub(crate) fn get_key(&self) -> &String {
        &self.api_key
    }

    pub(crate) fn get_secret(&self) -> &String {
        &self.api_secret
    }

    pub(crate) fn nonce() -> String {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let nonce = (duration.as_secs() * 1_000_000u64) + u64::from(duration.subsec_micros());

        (nonce as u64).to_string()
    }

    pub(crate) fn sign(&self, path: &str, nonce: &str, params: &str) -> String {
        let api_secret = base64::decode(&self.api_secret).unwrap();
        // Use base64 decoded API key as the HMAC key with Sha512 as the hashing function
        let mut hmac = HmacSha512::new_varkey(&api_secret).expect("Invalid API secret length");
        let mut sha256 = Sha256::new();

        // SHA256(nonce + POST data)
        sha256.update(nonce.as_bytes());
        sha256.update(params.as_bytes());
        let sha_res = sha256.finalize();

        hmac.update(path.as_bytes());
        hmac.update(&sha_res);

        base64::encode(hmac.finalize().into_bytes())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::api;
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
        let params = api::format_params(&Some(&params)).unwrap();

        let signature = auth.sign(&api_path, &api_nonce, &params);

        assert_eq!(signature, String::from("RdQzoXRC83TPmbERpFj0XFVArq0Hfadm0eLolmXTuN2R24hzIqtAnF/f7vSfW1tGt7xQOn8bjm+Ht+X0KrMwlA=="));
    }
}
