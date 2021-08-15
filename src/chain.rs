use crate::block::{Block, Transaction};

#[derive(Debug)]
pub struct Chain {
  blocks: Vec<Block>,
  transactions: Vec<Transaction>,
}

impl Chain {
  pub fn new() -> Self {
    let mut chain = Chain {
      blocks: Vec::new(),
      transactions: Vec::new(),
    };

    chain.block(100, "".to_owned());

    chain
  }
  pub fn transaction(&mut self, sender: String, recipient: String, amount: i64) -> usize {
    self.transactions.push(Transaction {
      sender,
      recipient,
      amount,
    });

    let last_block = self.blocks.last().unwrap();

    last_block.index + 1
  }

  fn block(&mut self, proof: usize, previous_block_hash: String) {
    self.blocks.push(Block {
      index: self.blocks.len() + 1,
      timestamp: 1,
      transactions: self.transactions.clone(),
      proof,
      previous_block_hash,
    });

    self.transactions.clear();
  }
}
