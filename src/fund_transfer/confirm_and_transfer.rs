// use super::{ ResponseCode, ProductType, ProductStatus};
use crate::{api_url, custom_serde::serialize_simple_uuid, default_merchant_id, generate_signature, platform_user_id, sign_hash, Error, ResponseCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmAndTransferResponse {
    pub code: ResponseCode,
    pub data: Option<TransferResponseStatus>,
    pub message: String,
}

/// BankInfoItemResponse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferResponseStatus {
    /// success
    pub success: Option<bool>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmAndTransferReqBody{
    /// merchantId
    pub merchant_id: String,
    /// outPaymentBillNum : your payment bill number
    #[serde(serialize_with = "serialize_simple_uuid")]
    pub out_payment_bill_num: Uuid,
    /// verifyCode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_code: Option<String>,
    /// Verify Code Method : PHONE,EMAIL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_code_method: Option<String>,
}


pub async fn confirm_and_transfer(out_payment_bill_num: Uuid, merchant_id: Option<String>) -> Result<ConfirmAndTransferResponse, Error> {
    let path: Vec<String> = vec!["payment-bill/confirm-and-transfer".to_string()];

    let merchant_id = merchant_id.unwrap_or(default_merchant_id());

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let body = serde_json::to_string(&ConfirmAndTransferReqBody{
        out_payment_bill_num: out_payment_bill_num,
        merchant_id: merchant_id,
        verify_code: None,
        verify_code_method: None,
    })?;

    let signature = generate_signature(&body);

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("SIGNATURE", signature)
        .header("PLATFORM_USER_ID", platform_user_id())
        .body(body)
        .send()
        .await
        .map_err(|e| Error::ResponseError(e.to_string()))?;

    let body = res.text().await.unwrap();
 
    let result: ConfirmAndTransferResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;

    Ok(result)
}
