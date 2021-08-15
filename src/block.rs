use sha2::Digest;

#[derive(Debug, Clone)]
pub struct Transaction {
  pub sender: String,
  pub recipient: String,
  pub amount: i64,
}

#[derive(Debug)]
pub struct Block {
  pub index: usize,
  pub timestamp: usize,
  pub transactions: Vec<Transaction>,
  pub previous_block_hash: String,
  pub proof: usize,
}

impl Block {
  fn hash(&self) -> String {
    let block_as_string = format!("{:?}", self);

    format!("{:x}", sha2::Sha256::digest(block_as_string.as_bytes()))
  }
}
