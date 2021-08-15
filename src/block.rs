use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Sender {
  System,
  Client(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
  pub sender: Sender,
  pub recipient: String,
  pub amount: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
  pub index: usize,
  pub timestamp: u128,
  pub transactions: Vec<Transaction>,
  pub previous_block_hash: String,
  pub proof: usize,
}

impl Block {
  pub fn hash(&self) -> String {
    let block_as_string = format!("{:?}", self);

    format!("{:x}", sha2::Sha256::digest(block_as_string.as_bytes()))
  }
}
