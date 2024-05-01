use serde::{Deserialize, Serialize};
use thiserror::Error;

// DEV API URL
pub const DEV_API_URL: &str = "http://api-test.51qbiz.id/openapi";
// PROD API URL
pub const PROD_API_URL: &str = "https://api.51qbiz.id/openapi";


#[derive(Serialize, Deserialize, Error, Debug)]
pub enum Error {
    #[error("Response error: `{0}`")]
    ResponseError(String),

    #[error("Decryption error: `{0}`")]
    DecryptionError(String),

    #[error("Deserialization error: `{0}`")]
    DeserializationError(String),

    #[error("Serialization error: `{0}`")]
    SerializationError(String),

    #[error("unknown model error")]
    Unknown,
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        // Convert the `serde_json::Error` to `Error` here.
        // This is just an example. You need to replace it with your actual conversion code.
        Error::DeserializationError(err.to_string())
    }
}

// generate signature data body + api_secret
pub fn generate_signature(body: &String) -> String {
    let body_appended = &format!("{}{}", body, api_secret());
    sign_hash(&body_appended)
}
  

// api secret
pub fn api_secret() -> String {
    match std::env::var("QBIZ_API_SECRET") {
        Ok(val) => val,
        Err(_e) => panic!("QBIZ_API_SECRET is not set"),
    }
}

// PLATFORM_USER_ID
pub fn platform_user_id() -> String {
    match std::env::var("QBIZ_PLATFORM_USER_ID") {
        Ok(val) => val,
        Err(_e) => panic!("QBIZ_PLATFORM_USER_ID is not set"),
    }
}

pub fn default_merchant_id() -> String {
    match std::env::var("QBIZ_DEFAULT_MERCHANT_ID") {
        Ok(val) => val,
        Err(_e) => panic!("QBIZ_DEFAULT_MERCHANT_ID is not set"),
    }
}

pub fn sign_hash(body: &str) -> String {
    format!("{:x}", md5::compute(body))
}



// function to get the endpoint based on env DEV or PROD
pub fn api_url() -> &'static str {
  match std::env::var("QBIZ_API_ENV") {
      Ok(val) => {
          if val == "PROD" {
              PROD_API_URL
          } else {
              DEV_API_URL
          }
      }
      Err(_e) => panic!("QBIZ_API_ENV is not set"),
  }
}