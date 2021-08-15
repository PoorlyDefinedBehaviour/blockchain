use crate::chain::Chain;
use crate::proof_of_work;
use crate::viewmodel;
use reqwest;

fn is_chain_valid(chain: &Chain) -> bool {
  let mut previous_block = chain.blocks.first().unwrap();

  for block in chain.blocks.iter().skip(1) {
    if previous_block.hash() != block.previous_block_hash {
      return false;
    }

    if !proof_of_work::is_proof_valid(
      previous_block.proof,
      block.proof,
      &block.previous_block_hash,
    ) {
      return false;
    }

    previous_block = block;
  }

  true
}

pub async fn resolve_conflicts(chain: &Chain) -> Result<Chain, Box<dyn std::error::Error>> {
  let mut longest_chain = chain.clone();

  // TODO: make requests parallel
  for node in &chain.nodes {
    let url = format!("{}/chain", node.to_string());

    let chain_viewmodel = reqwest::get(url)
      .await?
      .json::<viewmodel::ChainViewModel>()
      .await?;

    let other_chain = Chain::from(chain_viewmodel);

    if other_chain.blocks.len() > chain.blocks.len() && is_chain_valid(&other_chain) {
      longest_chain = other_chain;
    }
  }

  dbg!(&longest_chain);

  Ok(longest_chain)
}
