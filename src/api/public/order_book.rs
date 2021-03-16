use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    EndpointInfo, Input, KAssetPair, KrakenInput, MethodType, MutateInput, Output, UpdateInput,
};

/// Request builder for the Get Order Book endpoint
pub struct KIOrderBook {
    params: IndexMap<String, String>,
}

impl KIOrderBook {
    /// Constructor returning a [KrakenInput] builder for the get order book endpoint.
    /// * `pair` is the asset pair to get market depth for
    pub fn build(pair: KAssetPair) -> Self {
        let order_book = KIOrderBook {
            params: IndexMap::new(),
        };
        order_book.update_pair(pair)
    }

    /// Update the asset pair to get market depth for
    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    /// Max number of asks/bids to return
    pub fn with_max(self, max: i64) -> Self {
        self.update_input("count", max.to_string())
    }
}

impl Input for KIOrderBook {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("Depth"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("Depth"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
    }
}

impl MutateInput for KIOrderBook {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOrderBook {}

/// Order book data
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOrderBookData {
    pub price: String,
    pub volume: String,
    pub timestamp: i64,
}

/// Order book data
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOrderDepthPair {
    /// Ask side array of [KOOrderBookData]
    pub asks: Vec<KOOrderBookData>,
    /// Bid side array of [KOOrderBookData]
    pub bids: Vec<KOOrderBookData>,
}

/// Response from the Get Order Book endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOrderBook {
    /// Map with the asset pair as the key and the pair's order book depth data as the value
    #[serde(flatten)]
    pub pair: HashMap<KAssetPair, KOOrderDepthPair>,
}

impl Output for KOOrderBook {}
