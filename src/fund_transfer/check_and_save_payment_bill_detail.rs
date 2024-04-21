// use super::{ ResponseCode, ProductType, ProductStatus};
use crate::{api_url, default_merchant_id, generate_signature, platform_user_id, sign_hash, Error, ResponseCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckAndSavePaymentBillDetailResponse {
    pub code: ResponseCode,
    pub data: Option<CheckAndSavePaymentBillDetail>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckAndSavePaymentBillDetail {
    /// outPaymentBillNum:your payment bill number
    #[serde(with = "uuid::serde::simple")]
    pub out_payment_bill_num: Uuid,
    /// outPaymentSubNum:your payment bill number detail
    #[serde(with = "uuid::serde::simple")]
    pub out_payment_sub_num: Uuid,
    /// paymentBillNum:system payment bill number
    pub payment_bill_num: Option<String>,
    /// paymentSubNum:system payment bill number detail
    pub payment_sub_num: Option<String>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckAndSavePaymentBillDetailReqBody{  
    /// bankAccount
    pub bank_account: String,
    /// bankName
    /// from '/openapi/common/bank-list'.bank_name
    pub bank_name: String,
    /// bankId
    /// from '/openapi/common/bank-list'.bank_id
    pub bank_id: String,
    /// merchantId
    pub merchant_id: String,
    /// outPaymentBillNum:your payment bill number
    #[serde(with = "uuid::serde::simple")]
    pub out_payment_bill_num: Uuid,
    /// outPaymentSubNum:your payment bill number detail
    #[serde(with = "uuid::serde::simple")]
    pub out_payment_sub_num: Uuid,
    /// paymentAmount
    pub payment_amount: Option<u64>,
    /// receivingName
    pub receiving_name: String,
    /// remark
    pub remark: Option<String>,
}

pub async fn check_and_save_payment_bill_detail(
    out_payment_bill_num: Uuid, 
    bank_account: String,
    bank_name: String,
    bank_id: String,
    receiving_name: String,
    payment_amount: u64,
    remark: Option<String>,
    merchant_id: Option<String>
  ) -> Result<CheckAndSavePaymentBillDetailResponse, Error> {
    let path: Vec<String> = vec!["payment-bill/check-and-save-payment-bill-detail".to_string()];

    let merchant_id = merchant_id.unwrap_or(default_merchant_id());

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let body = serde_json::to_string(&CheckAndSavePaymentBillDetailReqBody{
      bank_account: bank_account,
      bank_name: bank_name,
      bank_id: bank_id,
      merchant_id: merchant_id,
      out_payment_bill_num: out_payment_bill_num,
      out_payment_sub_num: out_payment_bill_num, // intentionally same as out_payment_bill_num
      payment_amount: Some(payment_amount),
      receiving_name: receiving_name,
      remark: remark,
    })?;

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

    let body = res.text().await.unwrap();
 
    let result: CheckAndSavePaymentBillDetailResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;

    Ok(result)
}
