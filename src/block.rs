use sha2::Digest;

#[derive(Debug)]
pub struct Block {
  pub previous_block_hash: String,
  pub data: String,
  pub hash: String,
}

impl Block {
  fn hash(data: &String, previous_block_hash: &String) -> String {
    let current_block_hash_contents = format!("{}{}", data, previous_block_hash);

    format!(
      "{:x}",
      sha2::Sha256::digest(current_block_hash_contents.as_bytes())
    )
  }

  pub fn new(data: String, previous_block_hash: String) -> Self {
    Block {
      hash: Block::hash(&data, &previous_block_hash),
      previous_block_hash,
      data,
    }
  }
}
