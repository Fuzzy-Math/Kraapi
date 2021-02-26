use serde::{Deserialize, Serialize};
use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{
    EndpointInfo, KrakenInput, MethodType,
};

// Traits
use super::{
    Input, MutateInput, 
    UpdateInput
};

/// Request builder for the Cancel All Open Orders endpoint 
pub struct KICancelAllOrders {
    params: IndexMap<String, String>,
}

impl KICancelAllOrders {
    pub fn build() -> KrakenInput {
        let cancelorders = KICancelAllOrders {
            params: IndexMap::new()
        };
        cancelorders.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let cancelorders = KICancelAllOrders {
            params: IndexMap::new()
        };
        cancelorders.finish_clone()
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KICancelAllOrders {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelAll") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
       info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelAll") },
       params: Some(newself.params.clone())
       },
       newself)
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

