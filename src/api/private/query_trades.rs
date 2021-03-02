use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, InputList, InputListItem, IntoInputList, MutateInput, UpdateInput};

pub use super::KOTradeData;

/// Request builder for the Query Trades Info endpoint
pub struct KITradesInfo {
    params: IndexMap<String, String>,
}

impl KITradesInfo {
    pub fn build(txid: String) -> Self {
        let trades_info = KITradesInfo {
            params: IndexMap::new(),
        };
        trades_info.with_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let trades_info = KITradesInfo {
            params: IndexMap::new(),
        };
        trades_info.with_item_list(txids)
    }

    pub fn update_transaction_list<T>(self, txids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        self.update_input("txid", String::from(""))
            .with_item_list(txids)
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KITradesInfo {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("QueryTrades"),
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
                    endpoint: String::from("QueryTrades"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KITradesInfo {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradesInfo {}

impl IntoInputList for KITradesInfo {
    fn list_name(&self) -> String {
        String::from("txid")
    }
}

impl InputListItem for KITradesInfo {
    type ListItem = String;
}

impl InputList for KITradesInfo {}

/// Response from the Query Trades Info endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradesInfo {
    /// Map with the trade's transaction ID as the key and the trade info as the value
    #[serde(flatten)]
    pub trades: HashMap<String, KOTradeData>,
}
