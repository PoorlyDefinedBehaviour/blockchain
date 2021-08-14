use sha2::Digest;

#[derive(Debug)]
struct Chain {
  pub blocks: Vec<Block>,
}

impl Chain {
  fn generate_genesis_block() -> Block {
    Block::new("genesis".to_owned(), "".to_owned())
  }

  pub fn new() -> Self {
    Chain {
      blocks: vec![Chain::generate_genesis_block()],
    }
  }

  pub fn add_block(&mut self, data: String) {
    let previous_block = self.blocks.last().unwrap();

    let block = Block::new(data, previous_block.hash.clone());

    self.blocks.push(block);
  }
}

#[derive(Debug)]
struct Block {
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

fn main() {
  let mut chain = Chain::new();

  chain.add_block("First block".to_owned());
  chain.add_block("Second block".to_owned());
  chain.add_block("Third block".to_owned());

  dbg!(&chain);
}
