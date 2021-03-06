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
pub struct Address {
    #[serde(rename = "address")]
    pub address: String,
}

impl Address {
    pub fn new(address: String) -> Address {
        Address {
            address: address,
        }
    }
}
