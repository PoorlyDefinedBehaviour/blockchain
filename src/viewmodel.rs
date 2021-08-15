use crate::block::{Block, Sender, Transaction};
use crate::chain::Chain;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::SocketAddr;

const SYSTEM_SENDER: &str = "System";

#[derive(Deserialize, Serialize)]
pub struct MineViewModel {
  pub message: String,
  pub block: BlockViewModel,
}

#[derive(Deserialize, Serialize)]
pub struct MessageViewModel {
  pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTransactionViewModel {
  pub sender: String,
  pub recipient: String,
  pub amount: i64,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionViewModel {
  pub sender: String,
  pub recipient: String,
  pub amount: i64,
}

impl From<Transaction> for TransactionViewModel {
  fn from(item: Transaction) -> Self {
    let sender = match item.sender {
      Sender::System => SYSTEM_SENDER.to_owned(),
      Sender::Client(client_id) => client_id,
    };

    TransactionViewModel {
      sender,
      recipient: item.recipient,
      amount: item.amount,
    }
  }
}

impl From<TransactionViewModel> for Transaction {
  fn from(item: TransactionViewModel) -> Self {
    let sender = match item.sender {
      sender if sender == *SYSTEM_SENDER => Sender::System,
      node_id => Sender::Client(node_id),
    };

    Transaction {
      sender,
      recipient: item.recipient,
      amount: item.amount,
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct ChainViewModel {
  pub blocks: Vec<BlockViewModel>,
  pub transactions: Vec<TransactionViewModel>,
  pub nodes: HashSet<SocketAddr>,
}

impl From<Chain> for ChainViewModel {
  fn from(item: Chain) -> Self {
    ChainViewModel {
      blocks: item.blocks.into_iter().map(BlockViewModel::from).collect(),
      transactions: item
        .transactions
        .into_iter()
        .map(TransactionViewModel::from)
        .collect(),
      nodes: item.nodes,
    }
  }
}

impl From<ChainViewModel> for Chain {
  fn from(item: ChainViewModel) -> Self {
    Chain {
      blocks: item.blocks.into_iter().map(Block::from).collect(),
      transactions: item
        .transactions
        .into_iter()
        .map(Transaction::from)
        .collect(),
      nodes: item.nodes,
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct BlockViewModel {
  pub index: usize,
  pub timestamp: u128,
  pub transactions: Vec<TransactionViewModel>,
  pub previous_block_hash: String,
  pub proof: usize,
}

impl From<Block> for BlockViewModel {
  fn from(item: Block) -> Self {
    BlockViewModel {
      index: item.index,
      timestamp: item.timestamp,
      transactions: item
        .transactions
        .into_iter()
        .map(TransactionViewModel::from)
        .collect(),
      previous_block_hash: item.previous_block_hash,
      proof: item.proof,
    }
  }
}

impl From<BlockViewModel> for Block {
  fn from(item: BlockViewModel) -> Self {
    Block {
      index: item.index,
      timestamp: item.timestamp,
      transactions: item
        .transactions
        .into_iter()
        .map(Transaction::from)
        .collect(),
      previous_block_hash: item.previous_block_hash,
      proof: item.proof,
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct AddNodeToNetworkViewModel {
  pub address: String,
}
