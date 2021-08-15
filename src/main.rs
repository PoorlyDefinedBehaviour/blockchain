pub mod block;
pub mod chain;
pub mod controller;
pub mod proof_of_work;

use chain::Chain;

use std::sync::RwLock;
use uuid::Uuid;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let chain = web::Data::new(RwLock::new(Chain::new()));

  HttpServer::new(move || {
    App::new()
      .app_data(chain.clone())
      .app_data(web::Data::new(format!("{}", Uuid::new_v4())))
      .service(controller::mine)
      .service(controller::create_transaction)
      .service(controller::get_chain)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
