/* 
 * Skycoin REST API.
 *
 * Skycoin is a next-generation cryptocurrency.
 *
 * OpenAPI spec version: 0.25.1
 * Contact: contact@skycoin.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Apiv1walletsEntries {
  #[serde(rename = "public_key")]
  public_key: Option<String>,
  #[serde(rename = "address")]
  address: Option<String>
}

impl Apiv1walletsEntries {
  pub fn new() -> Apiv1walletsEntries {
    Apiv1walletsEntries {
      public_key: None,
      address: None
    }
  }

  pub fn set_public_key(&mut self, public_key: String) {
    self.public_key = Some(public_key);
  }

  pub fn with_public_key(mut self, public_key: String) -> Apiv1walletsEntries {
    self.public_key = Some(public_key);
    self
  }

  pub fn public_key(&self) -> Option<&String> {
    self.public_key.as_ref()
  }

  pub fn reset_public_key(&mut self) {
    self.public_key = None;
  }

  pub fn set_address(&mut self, address: String) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: String) -> Apiv1walletsEntries {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&String> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

}



