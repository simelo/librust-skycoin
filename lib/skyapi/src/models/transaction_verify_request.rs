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
pub struct TransactionVerifyRequest {
    #[serde(rename = "unsigned")]
    pub unsigned: Option<bool>,
    #[serde(rename = "encoded_transaction")]
    pub encoded_transaction: Option<String>,
}

impl TransactionVerifyRequest {
    pub fn new() -> TransactionVerifyRequest {
        TransactionVerifyRequest {
            unsigned: None,
            encoded_transaction: None,
        }
    }
}