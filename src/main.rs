pub mod block;
pub mod chain;

use chain::Chain;

fn main() {
  let mut chain = Chain::new();

  chain.add_block("First block".to_owned());
  chain.add_block("Second block".to_owned());
  chain.add_block("Third block".to_owned());

  dbg!(&chain);
}
