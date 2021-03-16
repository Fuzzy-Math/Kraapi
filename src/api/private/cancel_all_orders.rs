use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

/// Request builder for the Cancel All Open Orders endpoint
pub struct KICancelAllOrders {
    params: IndexMap<String, String>,
}

impl KICancelAllOrders {
    /// Construct and build a [KrakenInput] for the cancel all orders endpoint. No parameters are
    /// needed
    pub fn build() -> KrakenInput {
        let cancelorders = KICancelAllOrders {
            params: IndexMap::new(),
        };
        cancelorders.finish()
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KICancelAllOrders {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("CancelAll"),
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
                    endpoint: String::from("CancelAll"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KICancelAllOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KICancelAllOrders {}

/// Response from the Cancel All Open Orders endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOCancelAllOrders {
    /// number of orders canceled
    pub count: u32,
}

impl Output for KOCancelAllOrders {}
