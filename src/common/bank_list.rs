use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::{api_url, generate_signature, platform_user_id, sign_hash, Error, ResponseCode};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankListResponse {
    pub code: ResponseCode,
    pub data: Option<Vec<BankInfo>>,
    pub message: String,
}

/// BankInfoItemResponse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankInfo {
    /// bankId
    pub bank_id: Option<String>,
    /// bankName
    pub bank_name: Option<String>,
}


// post request to get the bank_list.
pub async fn bank_list() -> Result<BankListResponse, Error> {
    let path: Vec<String> = vec!["common/bank-list".to_string()];

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let body = "".to_string();

    let signature = generate_signature(&body);
    // send this with the intent to respond in json
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("SIGNATURE", signature)
        .header("PLATFORM_USER_ID", platform_user_id())
        .body(body)
        .send()
        .await
        .map_err(|e| Error::ResponseError(e.to_string()))?;

    let body = res.text().await.map_err(|e| Error::ResponseError(e.to_string()))?;
    let result: BankListResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;

    Ok(result)
}
