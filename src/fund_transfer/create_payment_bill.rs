// use super::{ ResponseCode, ProductType, ProductStatus};
use crate::{api_url, default_merchant_id, generate_signature, platform_user_id, sign_hash, Error, ResponseCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentBillResponse {
    pub code: ResponseCode,
    pub data: Option<PaymentBill>,
    pub message: String,
}

/// BankInfoItemResponse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentBill {
    /// outPaymentBillNum : your payment bill number
    #[serde(with = "uuid::serde::simple")]
    pub out_payment_bill_num: Uuid,
    /// paymentBillNum: system paymentBillNum
    pub payment_bill_num: String,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentBillReqBody{  
    /// feeDeductType:BALANCE,ORDER
    /// BALANCE:Deduction of handling fee from total balance
    /// ORDER:Deduction of handling fee from payment bill detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_deduct_type: Option<String>,
    /// merchantId
    pub merchant_id: String,
    /// outPaymentBillNum: your payment bill number
    #[serde(with = "uuid::serde::simple")]
    pub out_payment_bill_num: Uuid,
}


pub async fn create_payment_bill(out_payment_bill_num: Uuid, merchant_id: Option<String>) -> Result<PaymentBillResponse, Error> {
    let path: Vec<String> = vec!["payment-bill/create-payment-bill".to_string()];

    let merchant_id = merchant_id.unwrap_or(default_merchant_id());

    let url = Path::new(api_url())
        .join(path.join("/"))
        .to_str()
        .unwrap()
        .to_owned();
    let client = reqwest::Client::new();

    let body = serde_json::to_string(&PaymentBillReqBody{
        fee_deduct_type: None,
        merchant_id: merchant_id,
        out_payment_bill_num: out_payment_bill_num,
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
 
    let result: PaymentBillResponse = serde_json::from_str(&body).map_err(|e| Error::DeserializationError(e.to_string()))?;

    Ok(result)
}
