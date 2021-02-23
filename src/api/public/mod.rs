//! Module for interacting with Kraken's public API endpoints
//! # Note
//! Each type prefixed with "KI" is a KrakenInput builder which will build requests for the given
//! endpoint.
//! Each type postfixed with "KO" is the output object that has been returned from Kraken's servers
//! and has been parsed into the given structure

use super::{
    AssetPairInfo, InputList, InputListItem,
    EndpointInfo, Input, KAsset, 
    KAssetPair, KrakenInput, 
    IntoInputList, MutateInput, OHLCInterval, 
    MethodType, SystemStatus, UpdateInput
};

/// Get server time endpoint
pub mod server_time;

/// Get system status endpoint
pub mod system_status;

/// Get asset info endpoint
pub mod asset_info;

/// Get tradeable asset pairs endpoint
pub mod asset_pairs;

/// Get ticker info endpoint
pub mod ticker;

/// Get OHLC data endpoint
pub mod ohlc;

/// Get order book endpoint
pub mod order_book;

/// Get recent trades endpoint
pub mod recent_trades;

/// Get recent spread data endpoint
pub mod spread_data;

