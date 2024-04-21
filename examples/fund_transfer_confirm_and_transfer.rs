use std::{fmt::Error, str::FromStr};

use qbiz_api::fund_transfer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let payment_bill_id = Uuid::from_str("69c2e1dc-60b6-54ca-ae81-d98313bb216a").unwrap();
  let result = fund_transfer::confirm_and_transfer(payment_bill_id, None).await.unwrap();
  println!("confirm_and_transfer {:?}", result );
  Ok(())
}