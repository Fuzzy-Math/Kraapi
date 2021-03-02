use serde::{Deserialize, Serialize};

use super::{EndpointInfo, Input, KrakenInput, MethodType, SystemStatus};

/// Request builder for the Get System Status endpoint
pub struct KISystemStatus();

impl KISystemStatus {
    pub fn build() -> KrakenInput {
        let status = KISystemStatus();
        status.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let status = KISystemStatus();
        status.finish_clone()
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
