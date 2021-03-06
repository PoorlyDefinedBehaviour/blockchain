use crate::block::{Block, Transaction};
use std::collections::HashSet;
use std::net::{AddrParseError, SocketAddr};

#[derive(Debug, Clone)]
pub struct Chain {
  pub blocks: Vec<Block>,
  pub transactions: Vec<Transaction>,
  pub nodes: HashSet<SocketAddr>,
}

impl Chain {
  pub fn new() -> Self {
    Chain {
      blocks: vec![Block {
        index: 0,
        timestamp: std::time::SystemTime::now()
          .duration_since(std::time::SystemTime::UNIX_EPOCH)
          .unwrap()
          .as_millis(),
        transactions: Vec::new(),
        proof: 100,
        previous_block_hash: "".to_owned(),
      }],
      transactions: Vec::new(),
      nodes: HashSet::new(),
    }
  }

  pub fn register_node(&mut self, address: &str) -> Result<(), AddrParseError> {
    let parsed_address = address.parse::<SocketAddr>()?;
    self.nodes.insert(parsed_address);
    Ok(())
  }

  pub fn transaction(&mut self, transaction: Transaction) -> usize {
    self.transactions.push(transaction);

    let last_block = self.blocks.last().unwrap();

    last_block.index + 1
  }

  pub fn block(&mut self, proof: usize) -> Block {
    let previous_block_hash = self.blocks.last().unwrap().hash();

    let block = Block {
      index: self.blocks.len(),
      timestamp: std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis(),
      transactions: self.transactions.clone(),
      proof,
      previous_block_hash,
    };

    self.blocks.push(block.clone());

    self.transactions.clear();

    block
  }
}
