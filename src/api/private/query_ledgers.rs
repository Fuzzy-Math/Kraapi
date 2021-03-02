use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, InputList, InputListItem, IntoInputList, MutateInput, UpdateInput};

pub use super::KOLedgerInfo;
pub use super::KOLedgers;

/// Request builder for the Query Ledgers endpoint
pub struct KIQueryLedgers {
    params: IndexMap<String, String>,
}

impl KIQueryLedgers {
    pub fn build(ledgerid: String) -> Self {
        let ledgers = KIQueryLedgers {
            params: IndexMap::new(),
        };
        ledgers.with_item(ledgerid)
    }

    pub fn build_with_list<T>(ledgerids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let ledgers = KIQueryLedgers {
            params: IndexMap::new(),
        };
        ledgers.with_item_list(ledgerids)
    }

    pub fn update_transaction_list<T>(self, ledgerids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        self.update_input("id", String::from(""))
            .with_item_list(ledgerids)
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIQueryLedgers {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("QueryLedgers"),
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
                    endpoint: String::from("QueryLedgers"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KIQueryLedgers {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIQueryLedgers {}

impl IntoInputList for KIQueryLedgers {
    fn list_name(&self) -> String {
        String::from("id")
    }
}

impl InputListItem for KIQueryLedgers {
    type ListItem = String;
}

impl InputList for KIQueryLedgers {}
