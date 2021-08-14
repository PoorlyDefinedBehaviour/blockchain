use crate::block::Block;

#[derive(Debug)]
pub struct Chain {
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
