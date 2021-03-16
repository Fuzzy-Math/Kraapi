use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};
use crate::api::asset::KAsset;

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

/// Request builder for the Get Account Balance endpoint
pub struct KIAccountBalance {
    params: IndexMap<String, String>,
}

impl KIAccountBalance {
    /// Constructor returning a [KrakenInput] builder for the get trade balance endpoint.
    /// There are no inputs to this endpoint so finish() is called for you
    pub fn build() -> KrakenInput {
        let account_balance = KIAccountBalance {
            params: IndexMap::new(),
        };
        account_balance.finish()
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIAccountBalance {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("Balance"),
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
                    endpoint: String::from("Balance"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KIAccountBalance {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIAccountBalance {}

/// Response from the Get Account Balance endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAccountBalance {
    /// Map with the asset as the key and the asset's current balance as the value
    #[serde(flatten)]
    pub balances: HashMap<KAsset, String>,
}

impl Output for KOAccountBalance {}
