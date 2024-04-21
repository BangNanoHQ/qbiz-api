use std::{fmt::Error, str::FromStr};

use qbiz_api::fund_transfer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let payment_bill_id = Uuid::from_str("9b15aade-db23-588b-b66c-7a62945956c7").unwrap();
  let result = fund_transfer::create_payment_bill(payment_bill_id, None).await.unwrap();
  println!("create_payment_bill {:?}", result );
  Ok(())
}