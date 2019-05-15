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
pub struct InlineResponse2007Data {
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl InlineResponse2007Data {
    pub fn new() -> InlineResponse2007Data {
        InlineResponse2007Data {
            version: None,
        }
    }
}
