use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    EndpointInfo, Input, KAssetPair, KrakenInput, MethodType, MutateInput, Output, UpdateInput,
};

/// Request builder for the Get Recent Trades endpoint
pub struct KIRecentTrades {
    params: IndexMap<String, String>,
}

impl KIRecentTrades {
    /// Constructor returning a [KrakenInput] builder for the get recent trades endpoint.
    /// * `pair` is the asset pair to query OHLC data for
    pub fn build(pair: KAssetPair) -> Self {
        let recent_trades = KIRecentTrades {
            params: IndexMap::new(),
        };
        recent_trades.update_pair(pair)
    }

    /// Update the asset pair to query OHLC data for
    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    /// Unix timestamp to query OHLC data from. The [KORecentTrades] member `last` can be used as input to
    /// `since()` to query trades data since the last time data was requested
    pub fn since(self, id: String) -> Self {
        self.update_input("since", id)
    }
}

impl Input for KIRecentTrades {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("Trades"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("Trades"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
    }
}

impl MutateInput for KIRecentTrades {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIRecentTrades {}

/// Recent trade info data
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeInfo {
    pub price: String,
    pub volume: String,
    pub time: f64,
    pub tradetype: String,
    pub ordertype: String,
    pub misc: String,
}

/// Response from the Get Recent Trades endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KORecentTrades {
    /// Map with the asset pair as the key and the pair's Recent Trade data as the value
    #[serde(flatten)]
    pub pair: HashMap<KAssetPair, Vec<KOTradeInfo>>,
    /// ID to be used as "since" input to subsequent Trade Data requests
    pub last: String,
}

impl Output for KORecentTrades {}
