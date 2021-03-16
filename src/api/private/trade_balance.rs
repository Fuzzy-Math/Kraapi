use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KAsset, KrakenInput, MethodType};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

/// Request builder for the Get Trade Balance endpoint
pub struct KITradeBalance {
    params: IndexMap<String, String>,
}

impl KITradeBalance {
    /// Constructor returning a [KrakenInput] builder for the get trade balance endpoint.
    pub fn build() -> KITradeBalance {
        KITradeBalance {
            params: IndexMap::new(),
        }
    }

    /// Base asset to determine balance for. Default to USD
    pub fn with_asset(self, asset: KAsset) -> Self {
        self.update_input("asset", asset.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KITradeBalance {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeBalance {}

impl Input for KITradeBalance {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("TradeBalance"),
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
                    endpoint: String::from("TradeBalance"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

/// Response from the Get Trade Balance endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeBalance {
    /// cost basis of open positions
    pub c: String,
    /// equity = trade balance + unrealized net profit/loss
    pub e: String,
    /// equivalent balance (combined balance of all currencies)
    pub eb: String,
    /// margin amount of open positions
    pub m: String,
    /// free margin = equity - initial margin (maximum margin available to open new positions)
    pub mf: String,
    /// margin level = (equity / initial margin) * 100
    pub ml: String,
    /// unrealized net profit/loss of open positions
    pub n: String,
    /// trade balance (combined balance of all equity currencies)
    pub tb: String,
    /// current floating valuation of open positions
    pub v: String,
}

impl Output for KOTradeBalance {}
