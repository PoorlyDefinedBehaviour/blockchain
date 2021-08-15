use crate::block::{Block, Sender, Transaction};
use crate::chain::Chain;

use crate::proof_of_work;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::sync::RwLock;

use actix_web::{get, post, web, HttpResponse, Responder};

#[derive(Deserialize, Serialize)]
struct MineViewModel {
  pub message: String,
  pub block: BlockViewModel,
}

#[post("/mine")]
async fn mine(chain: web::Data<RwLock<Chain>>, node: web::Data<String>) -> impl Responder {
  let mut chain = chain.write().unwrap();

  let last_block = chain.blocks.last().unwrap();

  let proof = proof_of_work::mine(last_block.proof);

  chain.transaction(Transaction {
    sender: Sender::System,
    recipient: node.as_ref().clone(),
    amount: 1,
  });

  let block = chain.block(proof);

  HttpResponse::Ok().json(MineViewModel {
    message: "block forged".to_owned(),
    block: BlockViewModel::from(block),
  })
}

#[derive(Deserialize, Serialize)]
struct MessageResponseViewModel {
  pub message: String,
}

#[derive(Deserialize, Serialize)]
struct CreateTransactionViewModel {
  pub sender: String,
  pub recipient: String,
  pub amount: i64,
}

#[post("/transactions")]
async fn create_transaction(
  chain: web::Data<RwLock<Chain>>,
  transaction_viewmodel: web::Json<CreateTransactionViewModel>,
) -> impl Responder {
  let mut chain = chain.write().unwrap();

  let transaction_viewmodel = transaction_viewmodel.into_inner();

  let transaction = Transaction {
    sender: Sender::Client(transaction_viewmodel.sender),
    recipient: transaction_viewmodel.recipient,
    amount: transaction_viewmodel.amount,
  };

  let block_index = chain.transaction(transaction);

  let message = format!("transaction will be added to block {}", block_index);

  HttpResponse::Ok().json(MessageResponseViewModel { message })
}

#[derive(Deserialize, Serialize)]
struct TransactionViewModel {
  pub sender: String,
  pub recipient: String,
  pub amount: i64,
}

impl From<Transaction> for TransactionViewModel {
  fn from(item: Transaction) -> Self {
    let sender = match item.sender {
      Sender::System => "System".to_owned(),
      Sender::Client(client_id) => client_id,
    };

    TransactionViewModel {
      sender,
      recipient: item.recipient,
      amount: item.amount,
    }
  }
}

#[derive(Deserialize, Serialize)]
struct BlockViewModel {
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
        .iter()
        .map(|t| TransactionViewModel::from(t.clone()))
        .collect(),
      previous_block_hash: item.previous_block_hash,
      proof: item.proof,
    }
  }
}

#[derive(Deserialize, Serialize)]
struct GetChainViewModel {
  pub length: usize,
  pub blocks: Vec<BlockViewModel>,
}

#[get("/chain")]
async fn get_chain(data: web::Data<RwLock<Chain>>) -> impl Responder {
  let chain = data.read().unwrap();

  HttpResponse::Ok().json(GetChainViewModel {
    length: chain.blocks.len(),
    blocks: chain
      .blocks
      .iter()
      .map(|block| BlockViewModel::from(block.clone()))
      .collect(),
  })
}

#[post("/nodes")]
async fn add_node_to_network(data: web::Data<RwLock<Chain>>) -> impl Responder {
  let chain = data.read().unwrap();

  HttpResponse::Ok().json(GetChainViewModel {
    length: chain.blocks.len(),
    blocks: chain
      .blocks
      .iter()
      .map(|block| BlockViewModel::from(block.clone()))
      .collect(),
  })
}

#[post("/nodes/resolve")]
async fn resolve_nodes_conflict(data: web::Data<RwLock<Chain>>) -> impl Responder {
  let chain = data.read().unwrap();

  HttpResponse::Ok().json(GetChainViewModel {
    length: chain.blocks.len(),
    blocks: chain
      .blocks
      .iter()
      .map(|block| BlockViewModel::from(block.clone()))
      .collect(),
  })
}
