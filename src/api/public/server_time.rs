use serde::{Serialize, Deserialize};

use super::{
    EndpointInfo, Input, KrakenInput, 
    MethodType
};

/// Request builder for the Get Server Time endpoint 
pub struct KIServerTime();

impl KIServerTime {
    /// Here is more details
    pub fn build() -> KrakenInput {
        let time = KIServerTime();
        time.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let time = KIServerTime();
        time.finish_clone()
    }
}

impl Input for KIServerTime {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Time") },
           params: None 
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Time") },
           params: None 
        },
        self)
    }
}

/// Response from the Get Server Time endpoint 
#[derive(Deserialize, Serialize, Debug)]
pub struct KOServerTime {
    /// as unix timestamp
    pub unixtime: u64,
    /// as RFC 1123 time format
    pub rfc1123: String,
}

