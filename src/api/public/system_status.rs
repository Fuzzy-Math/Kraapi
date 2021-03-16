use serde::{Deserialize, Serialize};

use super::{EndpointInfo, Input, KrakenInput, MethodType, Output, SystemStatus};

/// Request builder for the Get System Status endpoint
pub struct KISystemStatus();

impl KISystemStatus {
    /// Constructor returning a [KrakenInput] builder for the get server time endpoint.
    /// There are no inputs to this endpoint so finish() is called for you
    pub fn build() -> KrakenInput {
        let status = KISystemStatus();
        status.finish()
    }
}

impl Input for KISystemStatus {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("SystemStatus"),
            },
            params: None,
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("SystemStatus"),
                },
                params: None,
            },
            self,
        )
    }
}

/// Response from the Get System Status endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOSystemStatus {
    /// Current system status or trading mode
    pub status: SystemStatus,
    /// Server time
    pub timestamp: String,
}

impl Output for KOSystemStatus {}
