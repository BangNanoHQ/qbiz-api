use std::{fmt::Error, str::FromStr};

use qbiz_api::fund_transfer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let payment_bill_id = Uuid::from_str("cb24dd9e-f524-5a3e-8bb6-9061a5a07e09").unwrap();
  let result = fund_transfer::create_payment_bill(payment_bill_id, None).await.unwrap();
  println!("create_payment_bill {:?}", result );
  Ok(())
}