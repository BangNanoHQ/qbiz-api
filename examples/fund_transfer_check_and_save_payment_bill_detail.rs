use std::{fmt::Error, os::unix::raw::dev_t, str::FromStr};

use qbiz_api::fund_transfer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let payment_bill_id = Uuid::from_str("0a850a98-383d-5ad9-8477-1fa3bdffaf9d").unwrap();
  let bank_account = "5859459100023141".to_string();
  let bank_name = "BANK NEO COMMERCE".to_string();
  let bank_id = "202304281438059424".to_string();
  let receiving_name = "SATUNOL".to_string();
  let payment_amount:u64 = 10000;
  let remark = Some("test1".to_string());

  let result = fund_transfer::check_and_save_payment_bill_detail(
    payment_bill_id, 
    bank_account,
    bank_name,
    bank_id,
    receiving_name,
    payment_amount,
    remark,
    None
  ).await.unwrap();
  println!("check_and_save_payment_bill_detail {:?}", result );
  Ok(())
}