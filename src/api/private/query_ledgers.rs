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
    /// Constructor returning a [KrakenInput] builder for the query ledgers endpoint.
    /// * `ledgerid` is the ledger ID to query info for
    pub fn build(ledgerid: String) -> Self {
        let ledgers = KIQueryLedgers {
            params: IndexMap::new(),
        };
        ledgers.with_item(ledgerid)
    }

    /// Constructor returning a [KrakenInput] builder for the query ledgers endpoint.
    /// * `ledgerids` is any iterable collection of ledger IDs to query info for
    pub fn build_with_list<T>(ledgerids: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let ledgers = KIQueryLedgers {
            params: IndexMap::new(),
        };
        ledgers.with_item_list(ledgerids)
    }

    /// Update the list of ledger IDs to query info for.
    /// Useful for templating
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let ledgers = KIQueryLedgers::build(String::from("L4UESK-KG3EQ-UFO4T5"));

        assert_eq!(ledgers.params.get("id").unwrap(), "L4UESK-KG3EQ-UFO4T5");
    }

    #[test]
    fn build_with_list () {
        let ledg_ids = vec!{ String::from("L4UESK-KG3EQ-UFO4T5"), String::from("L4UESK-KG3EQ-UFO4T6") };
        let ledgers = KIQueryLedgers::build_with_list(ledg_ids.clone());

        assert_eq!(ledgers.params.get("id").unwrap(), "L4UESK-KG3EQ-UFO4T5,L4UESK-KG3EQ-UFO4T6");
    }

    #[test]
    fn update_transaction_list() {
        let ledg_ids = vec!{ String::from("L4UESK-KG3EQ-UFO4T5"), String::from("L4UESK-KG3EQ-UFO4T6") };
        let ledgers = KIQueryLedgers::build_with_list(ledg_ids);

        let ledg_ids = vec!{ String::from("L4UESK-KG3EQ-UFO4T6"), String::from("L4UESK-KG3EQ-UFO4T7") };
        let ledgers = ledgers.update_transaction_list(ledg_ids.clone());

        assert_eq!(ledgers.params.get("id").unwrap(), 
                 &ledg_ids.into_iter().reduce(|acc, x| format!("{},{}", acc, &x)).unwrap()

        );
    }
}
