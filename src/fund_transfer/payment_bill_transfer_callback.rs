use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::vec::Vec;
use crate::fmt::optional_datetime_format;

use super::FeeDeductType;

#[derive(Deserialize, Debug, Clone)]
pub struct PaymentBillCallback {
    pub merchant_id: Option<String>,
    pub payment_bill_num: Option<String>,
    pub out_payment_bill_num: Option<String>,
    pub payment_status: Option<String>,
    pub payment_amount: Option<f64>,
    pub payment_fee: Option<f64>,
    pub balance_amount: Option<f64>,
    pub payment_config_fee: Option<f64>,
    pub fee_deduct_type: Option<FeeDeductType>,
    #[serde(with = "optional_datetime_format")]
    pub finish_time: Option<NaiveDateTime>,
    pub list: Option<Vec<PaymentBillSubCallback>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PaymentBillSubCallback {
    pub payment_bill_num: Option<String>,
    pub payment_sub_num: Option<String>,
    pub out_payment_sub_num: Option<String>,
    pub receiving_name: Option<String>,
    pub receiving_bank_id: Option<String>,
    pub receiving_bank_name: Option<String>,
    pub receiving_account: Option<String>,
    pub credited_amount: Option<f64>,
    pub balance_amount: Option<f64>,
    pub payment_fee: Option<f64>,
    pub payment_status: Option<String>,
    pub error_msg: Option<String>,
    pub receipt: Option<String>,
}