use crate::chain::Chain;
use crate::proof_of_work;

pub fn is_chain_valid(chain: &Chain) -> bool {
  let mut previous_block = &chain.blocks[0];

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

pub fn resolve_conflicts(chain: &Chain) {}
