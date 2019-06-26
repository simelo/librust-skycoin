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
pub struct TransactionEncodedS {
    #[serde(rename = "time")]
    pub time: Option<i64>,
    #[serde(rename = "status")]
    pub status: Option<::models::TransactionStatus>,
    #[serde(rename = "encoded_transaction")]
    pub encoded_transaction: Option<String>,
}

impl TransactionEncodedS {
    pub fn new() -> TransactionEncodedS {
        TransactionEncodedS {
            time: None,
            status: None,
            encoded_transaction: None,
        }
    }
}