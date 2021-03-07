use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    EndpointInfo, Input, KAssetPair, KrakenInput, MethodType, MutateInput, OHLCInterval, Output,
    UpdateInput,
};

/// Request builder for the Get OHLC Data endpoint
pub struct KIOHLC {
    pub(crate) params: IndexMap<String, String>,
}

impl KIOHLC {
    pub fn build(pair: KAssetPair) -> Self {
        let ohlc = KIOHLC {
            params: IndexMap::new(),
        };
        ohlc.update_pair(pair)
    }

    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn with_interval(self, interval: OHLCInterval) -> Self {
        self.update_input("interval", interval.to_string())
    }

    pub fn since(self, id: String) -> Self {
        self.update_input("since", id)
    }
}

impl Input for KIOHLC {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("OHLC"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("OHLC"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
    }
}

impl MutateInput for KIOHLC {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOHLC {}

/// OHLC info data
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOHLCData {
    pub timestamp: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub vwap: String,
    pub volume: String,
    pub count: i64,
}

/// Response from the Get OHLC Data endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOHLC {
    /// Map with the asset pair as the key and the pair's OHLC data as the value
    #[serde(flatten)]
    pub pair: HashMap<KAssetPair, Vec<KOOHLCData>>,
    /// ID to be used as "since" input to subsequent OHLC requests
    pub last: i64,
}

impl Output for KOOHLC {}
