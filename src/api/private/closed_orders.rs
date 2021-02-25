use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{
    EndpointInfo, KAsset,
    KAssetPair, KrakenInput,
    LedgerType, MethodType,
    OrderCloseTime, OrderType,
    OrderFlags, TradeHistoryType, 
    TradeType
};

// Traits
use super::{
    InputList, InputListItem, Input, 
    IntoInputList, MutateInput, 
    UpdateInput
};

pub use super::KOOrderDescription;
pub use super::KOOrderStatus;
pub use super::KOOrderInfo;

/// Request builder for the Get Closed Orders endpoint 
pub struct KIClosedOrders {
    params: IndexMap<String, String>,
}

impl KIClosedOrders {
    pub fn build() -> Self {
        KIClosedOrders {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    pub fn with_userref (self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    pub fn from_timestamp(self, timestamp: u64) -> Self {
        self.update_input("start", timestamp.to_string())
    }

    pub fn to_timestamp(self, timestamp: u64) -> Self {
        self.update_input("end", timestamp.to_string())
    }

    pub fn from_txid(self, txid: String) -> Self {
        self.update_input("start", txid)
    }

    pub fn to_txid(self, txid: String) -> Self {
        self.update_input("end", txid)
    }

    pub fn with_offset(self, offset: u64) -> Self {
        self.update_input("ofs", offset.to_string())
    }

    pub fn with_closetime(self, closetime: OrderCloseTime) -> Self {
        self.update_input("closetime", closetime.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIClosedOrders {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("ClosedOrders") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("ClosedOrders") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIClosedOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIClosedOrders {}

/// Response from the Get Closed Orders endpoint 
#[derive(Deserialize, Serialize, Debug)]
pub struct KOClosedOrders {
    pub closed: HashMap<String, KOOrderInfo>,
    pub count: u32,
}

