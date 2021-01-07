use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Time {
    /// as unix timestamp
    pub unixtime: i64,
    /// as RFC 1123 time format
    pub rfc1123: String,
}

/// A currency asset
#[derive(Deserialize, Serialize, Debug)]
pub struct Asset {
    /// asset class
    pub aclass: String,
    /// alternate name
    pub altname: String,
    /// scaling decimal places for record keeping
    pub decimals: u32,
    /// scaling decimal places for output display
    pub display_decimals: u32,
}
