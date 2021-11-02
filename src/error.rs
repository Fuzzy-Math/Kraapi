//! Module encapsulating the error handling of internal errors and errors returned from Kraken

use std::error::Error;
use std::fmt;

use hyper::Error as HyperError;
use serde_json::Error as SerdeError;

/// Newtype wrapper around a vector of error values
#[derive(Debug)]
pub struct KrakenErrors<KError>(pub Vec<KError>);

impl Error for KrakenErrors<KError> {}

impl fmt::Display for KrakenErrors<KError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            if let Some((first, errors)) = self.0.split_first() {
                errors.iter().fold(first.to_string(), |acc, error| format!("{},{}", acc, error))
            } else {
                String::from("")
            }
        )
    }
}

/// Possible errors that could occur internally or errors that were returned from Kraken
#[derive(Debug)]
pub enum KError {
    /// Wrapper around [hyper::Error] when an internal http error has occurred
    HttpError(HyperError),

    /// Wrapper around [serde_json::Error] for when serde fails to parse the json into the output
    /// structure
    ParseError(SerdeError),

    /// Failed to parse into KAsset/KAssetPair
    AssetParseError,

    /// Invalid currency pair
    /// You can pull the complete list of our asset pairs from the AssetPairs public call
    /// and look for the pair name as the entry of the Json headers or by the parameter
    /// "altname": `https://api.kraken.com/0/public/AssetPairs`
    UnknownAssetPair,

    /// This error is returned when a method is called without the required parameters.
    /// For example, calling the QueryOrders method without specifying a valid transaction
    /// id (txid) parameter would cause the invalid arguments error to be returned.
    /// Calling a method with unnecessary parameters would still not return the
    /// invalid arguments error because the unnecessary parameters would simply be ignored.
    InvalidArguments,

    /// Permission denied errors are returned when the API client is attempting a task
    /// for which the API key does not have permission. For example, if an API client
    /// attempted to retrieve the account balance using an API key that was configured to
    /// allow trading access but not account management access, then the permission denied
    /// error would be returned. You can review your API keys and their settings
    /// (such as their permissions) via the Settings -> API tab of account management.
    PermissionDenied,

    /// This error is returned when the API key used for the call is either expired or disabled,
    /// please review the API key in your Settings -> API tab of account management or
    /// generate a new one and update your application.
    InvalidKey,

    /// The Invalid Key error occurs if either your API key or API secret are written
    /// incorrectly in your program or because the POST data used in the authentication
    /// and the POST data sent to the API do not match
    InvalidSignature,

    /// This error is returned when an invalid nonce is sent.
    /// Check your [nonce
    /// window](https://support.kraken.com/hc/en-us/articles/360001148023-What-is-a-Nonce-Window-)
    InvalidNonce,

    /// This error occurs when the [API call
    /// limits](https://support.kraken.com/hc/en-us/articles/206548367-What-is-the-API-call-rate-limit-)
    /// are exceeded
    APIRateLimit,

    /// While adding/canceling orders does not count against our standard API counter limits,
    /// these operations do have their own add/cancel order counter. This counter works in a
    /// way where the longer orders are left on the book, the more orders clients are able
    /// to add/cancel. After the error "EAPI:Rate limit exceeded", please wait ~15 min for
    /// being able to send new requests.
    OrderRateLimit,

    /// Temporary lockout error messages can occur if you had too many failed API calls
    /// or too many invalid nonce errors in a short period of time or invalid signatures.
    /// Even though these calls return an error, that error still counts against your API
    /// limits and may result in a temporary lockout.
    ///
    /// Temporary lockouts typically last approximately 15 minutes. If you are triggering
    /// several invalid nonce errors, please increase the nonce window as this can help
    /// reduce the frequency that these errors will occur. Please try to reduce the frequency
    /// of your private API calls also.
    TemporaryLockout,

    /// Opening new spot positions on margin has been temporarily suspended for trading
    /// engine maintenance. The feature will be making a return soon and you can follow
    /// along with [updates](status.kraken.com)
    ///
    /// Another reasons may be that spot positions on margin are not currently available
    /// for clients residing in certain countries. Please see this article for our
    /// [geographical restrictions](https://support.kraken.com/hc/en-us/articles/360001368823)
    OpenPosition,

    /// No [hedging](https://support.kraken.com/hc/en-us/articles/205367328-Hedging).
    /// Cannot open a long and short position for the same pair.
    ///
    /// If wishing to open a long and short position for the same currency, please
    /// choose different trading pairs with the same currency as the base or quote currency.
    /// Ex: short XBT/USD, long XBT/EUR.
    OpposingPosition,

    /// This error occurs when you have exceeded the margin allowance limits for your
    /// current verification level. Margin allowance limits for each currency varies
    /// based on your current verification level. Please refer to this support article for
    /// more information regarding [margin allowance limits](https://support.kraken.com/hc/en-us/articles/209238787-Margin-allowance-limits)
    MarginAllowanceExceeded,

    /// We have limited funds available for margin extensions. The "insufficient margin"
    /// message indicates that we are out of funds in the applicable margin pool for the
    /// time being. This can change at any time. You may be able to successfully place your
    /// order just seconds or minutes later, but high volume orders and orders placed during
    /// high volume times may take longer. Please accept our apologies for any inconvenience.
    /// For more [information](https://support.kraken.com/hc/en-us/articles/217696017-Insufficient-Margin)
    InsufficientMargin,

    /// You do not have the funds available to place this order. Please review your open
    /// positions and orders for items that may be holding up your funds
    InsufficientFunds,

    /// You have not met the minimum order volume for this asset.
    ///
    /// You can find more information about [minimum order sizes](https://support.kraken.com/hc/en-us/articles/205893708-What-is-the-minimum-order-size-volume-)
    OrderMinimum,

    /// You have exceeded the maximum amount of open orders available to your account.
    ///
    /// These limits are based on your verification level. Please close some of your open
    /// orders or verify your account to a higher level.
    ///
    /// You can learn more about the [maximum amount of open orders](https://support.kraken.com/hc/en-us/articles/209090607-What-is-the-maximum-number-of-open-orders-positions-)
    OrderLimit,

    /// You have exceeded the maximum amount of open positions available to your account.
    ///
    /// These limits are based on your verification level. Please close or settle some or all
    /// of your open positions or verify your account to a higher level if possible.
    ///
    /// You can learn more about the [maximum amount of open positions](https://support.kraken.com/hc/en-us/articles/209090607-What-is-the-maximum-number-of-open-orders-positions-)
    PositionLimit,

    /// In case of this error you will need to submit your order with the following parameter:
    /// ‘trading_agreement’:’agree’
    ///
    /// This will resolve the error message you are receiving when placing an order:
    /// [Trading Agreement](https://support.kraken.com/hc/en-us/articles/360000920026-Trading-Agreement-required-for-orders-sent-via-API)
    TradingAgreement,

    /// The service errors you are experiencing should only be temporary. You may wish to
    /// resubmit your requests if they have failed. We will be monitoring the issues and
    /// will update our [page](https://status.kraken.com/)
    ServiceUnavailable,

    /// The service errors you are experiencing should only be temporary. You may wish to
    /// resubmit your requests if they have failed. We will be monitoring the issues and
    /// will update our [page](https://status.kraken.com/)
    ServiceBusy,

    /// When we are facing API degradation issues, these can translate into problems for
    /// both Kraken and cryptowat.ch in the form of service unavailable messages, 8XX errors
    /// on [cryptowatch](https://cryptowat.ch/) and site outages.
    InternalError,

    /// This issue has to do with the security of your account which may have been compromised.
    /// Please change your password and Two-Factor Authentication and contact our Support Center
    Locked,

    /// This error occurs when a flag or input parameter is disabled temporary or permanently.
    /// The error should come from one of the inputs passed, please contact our support sending
    /// a log with the complete informations used for the call that generated the error
    FeatureDisabled,

    /// Default KError if none of the others above
    UnknownError,
}

impl fmt::Display for KError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Errors from internal dependencies
            KError::HttpError(err) => write!(f, "HTTP Error: {}", err.to_string()),
            KError::ParseError(err) => write!(f, "Parse Error: {}", err.to_string()),

            // Errors from processing within this crate
            KError::AssetParseError => write!(f, "Failed to parse string into KAsset"),

            // Errors coming directly from Kraken's servers
            KError::UnknownAssetPair => write!(f, "Unknown AssetPair"),
            KError::InvalidArguments => write!(f, "Invalid Arguments"),
            KError::PermissionDenied => write!(f, "Permission Denied"),
            KError::InvalidKey => write!(f, "Invalid Key"),
            KError::InvalidSignature => write!(f, "Invalid Signature"),
            KError::InvalidNonce => write!(f, "Invalid Nonce"),
            KError::APIRateLimit => write!(f, "API Rate Limit"),
            KError::OrderRateLimit => write!(f, "Order Rate Limit"),
            KError::TemporaryLockout => write!(f, "Temporary Lockout"),
            KError::OpenPosition => write!(f, "Cannot Open Position"),
            KError::OpposingPosition => write!(f, "Cannot Open Opposing Position"),
            KError::MarginAllowanceExceeded => write!(f, "Margin Allowance Exceeded"),
            KError::InsufficientMargin => write!(f, "Insufficient Margin"),
            KError::InsufficientFunds => write!(f, "Insufficient User Funds"),
            KError::OrderMinimum => write!(f, "Order Minimum Not Met (volume too low)"),
            KError::OrderLimit => write!(f, "Orders Limit Reached"),
            KError::PositionLimit => write!(f, "Positions Limit Reached"),
            KError::TradingAgreement => write!(f, "Trading Agreement Required"),
            KError::ServiceUnavailable => write!(f, "Service Unavailable"),
            KError::ServiceBusy => write!(f, "Service Busy"),
            KError::InternalError => write!(f, "Internal Error"),
            KError::Locked => write!(f, "Account Locked"),
            KError::FeatureDisabled => write!(f, "A Feature Was Disabled"),
            KError::UnknownError => write!(f, "An Unknown Error Occurred"),
        }
    }
}

impl From<HyperError> for KrakenErrors<KError> {
    fn from(err: HyperError) -> Self {
        KrakenErrors(vec![KError::HttpError(err)])
    }
}

impl From<SerdeError> for KrakenErrors<KError> {
    fn from(err: SerdeError) -> Self {
        KrakenErrors(vec![KError::ParseError(err)])
    }
}

pub(crate) fn generate_errors(errors: Vec<String>) -> KrakenErrors<KError> {
    let mut errs: Vec<KError> = Vec::with_capacity(errors.len());
    for error in errors {
        let index = error.find(':').unwrap();
        // Assume kraken will not return an error like "EError:" with no error description
        let (category, message) = error.split_at(index + 1);

        let err = match message {
            "Unknown asset pair" => KError::UnknownAssetPair,
            "Invalid arguments" => KError::InvalidArguments,
            "Permission denied" => KError::PermissionDenied,
            "Invalid key" => KError::InvalidKey,
            "Invalid signature" => KError::InvalidSignature,
            "Invalid nonce" => KError::InvalidNonce,
            "Rate limit exceeded" => match category {
                "EAPI:" => KError::APIRateLimit,
                "EOrder:" => KError::OrderRateLimit,
                _ => KError::UnknownError,
            },
            "Temporary lockout" => KError::TemporaryLockout,
            "Cannot open position" => KError::OpenPosition,
            "Cannot open opposing position" => KError::OpposingPosition,
            "Margin allowance exceeded" => KError::MarginAllowanceExceeded,
            "Insufficient margin" => KError::InsufficientMargin,
            "Insufficient insufficient user funds" => KError::InsufficientFunds,
            "Order minimum not volume too low" => KError::OrderMinimum,
            "Orders limit exceeded" => KError::OrderLimit,
            "Positions limit exceeded" => KError::PositionLimit,
            "Trading agreement required" => KError::TradingAgreement,
            "Unavailable" => KError::ServiceUnavailable,
            "Busy" => KError::ServiceBusy,
            "Internal error" => KError::InternalError,
            "Locked" => KError::Locked,
            "Feature disabled" => KError::FeatureDisabled,
            _ => KError::UnknownError,
        };

        errs.push(err);
    }

    KrakenErrors(errs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_kraken_errors() {
        let errors = KrakenErrors(vec![KError::UnknownAssetPair, KError::UnknownError]);
        let empty_errors = KrakenErrors(vec![]);

        assert_eq!(format!("{}", errors), String::from("[Unknown AssetPair,An Unknown Error Occurred]"));
        assert_eq!(format!("{}", empty_errors), String::from("[]"));
    }
}
