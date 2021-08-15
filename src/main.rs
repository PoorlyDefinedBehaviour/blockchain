pub mod block;
pub mod chain;
pub mod proof_of_work;

use block::{Block, Sender, Transaction};
use chain::Chain;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Deserialize, Serialize)]
struct MineResponse {
  pub message: String,
  pub index: usize,
  pub transactions: Vec<Transaction>,
  pub proof: usize,
  pub previous_block_hash: String,
}

#[post("/mine")]
async fn mine(chain: web::Data<Mutex<Chain>>, node: web::Data<String>) -> impl Responder {
  let mut chain = chain.lock().unwrap();

  let last_block = chain.blocks.last().unwrap();

  let proof = proof_of_work::mine(last_block.proof);

  chain.transaction(Transaction {
    sender: Sender::System,
    recipient: node.as_ref().clone(),
    amount: 1,
  });

  let block = chain.block(proof);

  HttpResponse::Ok().json(MineResponse {
    message: "block forged".to_owned(),
    index: block.index,
    transactions: block.transactions.clone(),
    proof: block.proof,
    previous_block_hash: block.previous_block_hash,
  })
}

#[derive(Deserialize, Serialize)]
struct MessageResponse {
  pub message: String,
}

#[post("/transactions")]
async fn create_transaction(
  data: web::Data<Mutex<Chain>>,
  transaction: web::Json<Transaction>,
) -> impl Responder {
  let mut chain = data.lock().unwrap();

  let block_index = chain.transaction(transaction.into_inner());

  let message = format!("transaction will be added to block {}", block_index);

  HttpResponse::Ok().json(MessageResponse { message })
}

#[derive(Deserialize, Serialize)]
struct GetChainResponse {
  pub length: usize,
  pub blocks: Vec<Block>,
}

#[get("/chain")]
async fn get_chain(data: web::Data<Chain>) -> impl Responder {
  let chain = data.as_ref();
  HttpResponse::Ok().json(GetChainResponse {
    length: chain.blocks.len(),
    blocks: chain.blocks.clone(),
  })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let chain = web::Data::new(Mutex::new(Chain::new()));

  HttpServer::new(move || {
    App::new()
      .app_data(chain.clone())
      .app_data(web::Data::new(format!("{}", Uuid::new_v4())))
      .service(mine)
      .service(create_transaction)
      .service(get_chain)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
