use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType, TradeHistoryType};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

pub use super::KOTradeData;

/// Request builder for the Get Trades History endpoint
pub struct KITradeHistory {
    params: IndexMap<String, String>,
}

impl KITradeHistory {
    pub fn build() -> Self {
        KITradeHistory {
            params: IndexMap::new(),
        }
    }

    pub fn with_trade_type(self, tradetype: TradeHistoryType) -> Self {
        self.update_input("type", tradetype.to_string())
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    pub fn starting_timestamp(self, timestamp: u64) -> Self {
        self.update_input("start", timestamp.to_string())
    }

    pub fn ending_timestamp(self, timestamp: u64) -> Self {
        self.update_input("end", timestamp.to_string())
    }

    pub fn starting_txid(self, txid: String) -> Self {
        self.update_input("start", txid)
    }

    pub fn ending_txid(self, txid: String) -> Self {
        self.update_input("end", txid)
    }

    pub fn with_offset(self, offset: u64) -> Self {
        self.update_input("ofs", offset.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KITradeHistory {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("TradesHistory"),
            },
            params: Some(self.with_nonce().params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        let newself = self.with_nonce();
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Private,
                    endpoint: String::from("TradesHistory"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KITradeHistory {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeHistory {}

/// Response from the Get Trades History endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeHistory {
    pub closed: HashMap<String, KOTradeData>,
    pub count: u32,
}

impl Output for KOTradeHistory {}
