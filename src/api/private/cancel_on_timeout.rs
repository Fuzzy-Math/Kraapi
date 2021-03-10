use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

/// Request builder for the Cancel All Orders After endpoint
pub struct KICancelOnTimeout {
    params: IndexMap<String, String>,
}

impl KICancelOnTimeout {
    /// Constructor returning a KrakenInput builder for the cancel all orders after... endpoint.
    /// Cancel all orders in `timeout` seconds
    pub fn build(timeout: u32) -> KICancelOnTimeout {
        let cancelorder = KICancelOnTimeout {
            params: IndexMap::new(),
        };
        cancelorder.on_timeout(timeout)
    }

    /// Update the timeout value. Useful for templating
    pub fn on_timeout(self, timeout: u32) -> Self {
        self.update_input("timeout", timeout.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KICancelOnTimeout {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KICancelOnTimeout {}

impl Input for KICancelOnTimeout {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("CancelAllOrdersAfter"),
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
                    endpoint: String::from("CancelAllOrdersAfter"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

/// Response from the Cancel All Orders After endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOCancelOnTimeout {
    /// Timestamp (RFC3339) reflecting when the request has been handled (second precision, rounded up)
    #[serde(rename = "currentTime")]
    pub current_time: String,
    /// Timestamp (RFC3339) reflecting the time at which all open orders will be cancelled,
    /// unless the timer is extended or disabled (second precision, rounded up)
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,
}

impl Output for KOCancelOnTimeout {}
