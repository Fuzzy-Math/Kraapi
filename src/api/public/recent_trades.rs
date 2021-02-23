use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use super::{
    EndpointInfo, Input, KAssetPair, 
    KrakenInput, MutateInput, MethodType, 
    UpdateInput
};

/// Request builder for the Get Recent Trades endpoint
pub struct KIRecentTrades {
    pub params: IndexMap<String, String>,
}

impl KIRecentTrades {
    pub fn build(pair: KAssetPair) -> Self {
        let recent_trades = KIRecentTrades {
            params: IndexMap::new()
        };
        recent_trades.update_pair(pair)
    }

    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn since(self, id: String) -> Self{
        self.update_input("since", id)
    }
}

impl Input for KIRecentTrades {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Trades") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Trades") },
           params: Some(self.params.clone())
       },
       self)
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
    pub pair: HashMap<String, Vec<KOTradeInfo>>,
    /// ID to be used as "since" input to subsequent Trade Data requests
    pub last: String,
}

