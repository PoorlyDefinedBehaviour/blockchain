use crate::block::{Sender, Transaction};
use crate::chain::Chain;
use crate::consensus;
use crate::proof_of_work;
use crate::viewmodel;
use std::sync::RwLock;

use actix_web::{get, post, web, HttpResponse, Responder};

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

  HttpResponse::Ok().json(viewmodel::MineViewModel {
    message: "block forged".to_owned(),
    block: viewmodel::BlockViewModel::from(block),
  })
}

#[post("/transactions")]
async fn create_transaction(
  chain: web::Data<RwLock<Chain>>,
  transaction_viewmodel: web::Json<viewmodel::CreateTransactionViewModel>,
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

  HttpResponse::Ok().json(viewmodel::MessageViewModel { message })
}

#[get("/chain")]
async fn get_chain(data: web::Data<RwLock<Chain>>) -> impl Responder {
  let chain = data.read().unwrap();

  HttpResponse::Ok().json(viewmodel::ChainViewModel::from(chain.clone()))
}

#[post("/nodes")]
async fn add_node_to_network(
  data: web::Data<RwLock<Chain>>,
  viewmodel: web::Json<viewmodel::AddNodeToNetworkViewModel>,
) -> impl Responder {
  let mut chain = data.write().unwrap();

  match chain.register_node(&viewmodel.address) {
    Err(_) => HttpResponse::BadRequest().json(viewmodel::MessageViewModel {
      message: format!("{} is not valid", viewmodel.address),
    }),
    Ok(()) => HttpResponse::Ok().json(viewmodel::MessageViewModel {
      message: format!("{} added to network", viewmodel.address),
    }),
  }
}

#[post("/nodes/resolve")]
async fn resolve_conflicts(data: web::Data<RwLock<Chain>>) -> impl Responder {
  let mut chain = data.write().unwrap();

  match consensus::resolve_conflicts(&chain).await {
    Err(_) => HttpResponse::InternalServerError(),
    Ok(longest_chain) => {
      chain.blocks = longest_chain.blocks;
      chain.transactions = longest_chain.transactions;
      chain.nodes = longest_chain.nodes;
      HttpResponse::Ok()
    }
  }
}
