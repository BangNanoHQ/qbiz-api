use std::fmt::Error;

use qbiz_api::common;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let result = common::bank_list().await.unwrap(); 
  println!("bank_list {:?}", result );
  Ok(())
}