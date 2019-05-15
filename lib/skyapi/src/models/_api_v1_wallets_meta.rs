/*
 * Skycoin REST API.
 *
 * Skycoin is a next-generation cryptocurrency.
 *
 * OpenAPI spec version: 0.25.1
 * Contact: contact@skycoin.net
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiV1WalletsMeta {
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    #[serde(rename = "encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "crypto_type")]
    pub crypto_type: Option<String>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "coin")]
    pub coin: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i32>,
}

impl ApiV1WalletsMeta {
    pub fn new() -> ApiV1WalletsMeta {
        ApiV1WalletsMeta {
            filename: None,
            encrypted: None,
            crypto_type: None,
            label: None,
            _type: None,
            version: None,
            coin: None,
            timestamp: None,
        }
    }
}
