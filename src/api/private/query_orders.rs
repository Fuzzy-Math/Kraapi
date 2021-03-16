use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, InputList, InputListItem, IntoInputList, MutateInput, Output, UpdateInput};

pub use super::KOOrderDescription;
pub use super::KOOrderInfo;
pub use super::KOOrderStatus;

/// Request builder for the Query Orders Info endpoint
pub struct KIQueryOrders {
    params: IndexMap<String, String>,
}

impl KIQueryOrders {
    /// Constructor returning a [KrakenInput] builder for the query orders info endpoint.
    /// * `txid` is the transaction ID to query order info for
    pub fn build(txid: String) -> Self {
        let order_info = KIQueryOrders {
            params: IndexMap::new(),
        };
        order_info.with_item(txid)
    }

    /// Constructor returning a [KrakenInput] builder for the query orders info endpoint.
    /// * `txids` is any iterable collection of transaction IDs to query order info for
    pub fn build_with_list<T>(txids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let order_info = KIQueryOrders {
            params: IndexMap::new(),
        };
        order_info.with_item_list(txids)
    }

    /// Update the list of transaction IDs to query order info for.
    /// Useful for templating
    pub fn update_transaction_list<T>(self, txids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        self.update_input("txid", String::from(""))
            .with_item_list(txids)
    }

    /// Should trades be included in returned output?
    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    /// Filter results to the given user ref id. 
    /// A custom userref can be passed into the add order endpoint
    pub fn with_userref(self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIQueryOrders {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("QueryOrders"),
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
                    endpoint: String::from("QueryOrders"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KIQueryOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIQueryOrders {}

impl IntoInputList for KIQueryOrders {
    fn list_name(&self) -> String {
        String::from("txid")
    }
}

impl InputListItem for KIQueryOrders {
    type ListItem = String;
}

impl InputList for KIQueryOrders {}

/// Response from the Query Orders Info endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOQueryOrders {
    /// Map with the order's transaction ID as the key and the order info as the value
    #[serde(flatten)]
    pub orders: HashMap<String, KOOrderInfo>,
}

impl Output for KOQueryOrders {}
