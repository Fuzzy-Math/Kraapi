use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KAsset, KrakenInput, LedgerType, MethodType};

// Traits
use super::{Input, InputList, InputListItem, IntoInputList, MutateInput, UpdateInput};

pub use super::KOLedgerInfo;
pub use super::KOLedgers;

/// Request builder for the Get Ledgers Info endpoint
pub struct KILedgerInfo {
    params: IndexMap<String, String>,
}

impl KILedgerInfo {
    /// Constructor returning a [KrakenInput] builder for the get ledgers info endpoint.
    pub fn build() -> Self {
        KILedgerInfo {
            params: IndexMap::new(),
        }
    }

    /// An asset is not required for the get ledgers info endpoint. 
    /// This method clears a currently populated asset list.
    /// Useful for templating
    pub fn clear_asset_list(self) -> Self {
        self.update_input("asset", String::from(""))
    }

    /// Update the list of assets to query info for 
    pub fn with_asset(self, asset: KAsset) -> Self {
        self.with_item(asset)
    }

    /// Update the list of assets to query info for 
    pub fn with_asset_list<T>(self, assets: T) -> Self
    where
        T: IntoIterator<Item = KAsset>,
    {
        self.with_item_list(assets)
    }

    /// Type of [Ledger Type][LedgerType] to retrieve
    pub fn with_trade_type(self, ledgertype: LedgerType) -> Self {
        self.update_input("type", ledgertype.to_string())
    }

    /// Starting Unix timestamp to filter output by. Exclusive
    pub fn starting_timestamp(self, timestamp: u64) -> Self {
        self.update_input("start", timestamp.to_string())
    }

    /// Ending Unix timestamp to filter output by. Inclusive
    pub fn ending_timestamp(self, timestamp: u64) -> Self {
        self.update_input("end", timestamp.to_string())
    }

    /// Starting ledger ID to filter output by. Exclusive
    pub fn starting_legid(self, legid: String) -> Self {
        self.update_input("start", legid)
    }

    /// Ending ledger ID to filter output by. Inclusive
    pub fn ending_legid(self, legid: String) -> Self {
        self.update_input("end", legid)
    }

    /// Result offset
    pub fn with_offset(self, offset: u64) -> Self {
        self.update_input("ofs", offset.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KILedgerInfo {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("Ledgers"),
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
                    endpoint: String::from("Ledgers"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KILedgerInfo {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KILedgerInfo {}

impl IntoInputList for KILedgerInfo {
    fn list_name(&self) -> String {
        String::from("asset")
    }
}

impl InputListItem for KILedgerInfo {
    type ListItem = KAsset;
}

impl InputList for KILedgerInfo {}
