use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::vec::Vec;

use super::FeeDeductType;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentBillCallback {
    pub merchant_id: Option<String>,
    pub payment_bill_num: Option<String>,
    pub out_payment_bill_num: Option<String>,
    pub payment_status: Option<String>,
    pub payment_amount: Option<i64>,
    pub payment_fee: Option<i64>,
    pub balance_amount: Option<i64>,
    pub payment_config_fee: Option<i64>,
    pub fee_deduct_type: Option<FeeDeductType>,
    pub finish_time: Option<NaiveDateTime>,
    pub list: Option<Vec<PaymentBillSubCallback>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentBillSubCallback {
    pub payment_bill_num: Option<String>,
    pub payment_sub_num: Option<String>,
    pub out_payment_sub_num: Option<String>,
    pub receiving_name: Option<String>,
    pub receiving_bank_id: Option<String>,
    pub receiving_bank_name: Option<String>,
    pub receiving_account: Option<String>,
    pub credited_amount: Option<i64>,
    pub balance_amount: Option<i64>,
    pub payment_fee: Option<i64>,
    pub payment_status: Option<String>,
    pub error_msg: Option<String>,
    pub receipt: Option<String>,
}