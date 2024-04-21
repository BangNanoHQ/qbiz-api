use std::{fmt::Error, os::unix::raw::dev_t, str::FromStr};

use qbiz_api::fund_transfer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let dev_payment_bill_id = Uuid::from_str("69c2e1dc-60b6-54ca-ae81-d98313bb216a").unwrap();
  let dev_bank_account = "5859459100023141".to_string();
  let dev_bank_name = "BANK NEO COMMERCE".to_string();
  let dev_bank_id = "202304281438059424".to_string();
  let dev_receiving_name = "SATUNOL".to_string();
  let dev_payment_amount:u64 = 10000;
  let dev_remark = Some("test1".to_string());


  let result = fund_transfer::check_and_save_payment_bill_detail(
    dev_payment_bill_id, 
    dev_bank_account,
    dev_bank_name,
    dev_bank_id,
    dev_receiving_name,
    dev_payment_amount,
    dev_remark,
    None
  ).await.unwrap();
  println!("check_and_save_payment_bill_detail {:?}", result );
  Ok(())
}