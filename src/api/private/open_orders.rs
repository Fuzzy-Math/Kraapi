use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, MutateInput, UpdateInput};

pub use super::KOOrderDescription;
pub use super::KOOrderInfo;
pub use super::KOOrderStatus;

/// Request builder for the Get Open Orders endpoint
pub struct KIOpenOrders {
    params: IndexMap<String, String>,
}

impl KIOpenOrders {
    pub fn build() -> Self {
        KIOpenOrders {
            params: IndexMap::new(),
        }
    }

    // FIXME: AFter testing, trades=false still causes trade data to be returned. So the entire key
    // value pair needs to be removed on false input
    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    pub fn with_userref(self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIOpenOrders {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("OpenOrders"),
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
                    endpoint: String::from("OpenOrders"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KIOpenOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOpenOrders {}

/// Response from the Get Open Orders endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOpenOrders {
    pub orders: HashMap<String, KOOrderInfo>,
}
