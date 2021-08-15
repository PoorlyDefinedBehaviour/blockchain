use sha2::Digest;

pub fn mine(last_proof: usize) -> usize {
  let mut nonce: usize = 0;

  loop {
    let guess = format!("{}{}", last_proof, nonce);

    let hash = format!("{:x}", sha2::Sha256::digest(guess.as_bytes()));

    // TODO: this is duplicated and it is not obvious
    // that it is connected to the mining difficulty
    if hash.ends_with("0000") {
      return nonce;
    }

    nonce += 1;
  }
}

pub fn is_proof_valid(last_proof: usize, proof: usize, previous_block_hash: &str) -> bool {
  let guess = format!("{}{}{}", last_proof, proof, previous_block_hash);

  let hash = format!("{:x}", sha2::Sha256::digest(guess.as_bytes()));

  hash.ends_with("0000")
}
