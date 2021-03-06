/*
 * Skycoin REST API.
 *
 * Skycoin is a next-generation cryptocurrency.
 *
 * The version of the OpenAPI document: 0.26.0
 * Contact: contact@skycoin.net
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionRequestHoursSelection {
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    #[serde(rename = "share_factor")]
    pub share_factor: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

impl WalletTransactionRequestHoursSelection {
    pub fn new() -> WalletTransactionRequestHoursSelection {
        WalletTransactionRequestHoursSelection {
            mode: None,
            share_factor: None,
            _type: None,
        }
    }
}
