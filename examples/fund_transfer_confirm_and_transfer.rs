use std::{fmt::Error, str::FromStr};

use qbiz_api::fund_transfer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let payment_bill_id = Uuid::from_str("e8226450-ad58-5683-971b-86b91eb23bb0").unwrap();
  let result = fund_transfer::confirm_and_transfer(payment_bill_id, None).await.unwrap();
  println!("confirm_and_transfer {:?}", result );
  Ok(())
}