use sha2::Digest;

pub fn mine(last_proof: usize) -> usize {
  let mut nonce: usize = 0;

  loop {
    let guess = format!("{}{}", last_proof, nonce);

    let hash = format!("{:x}", sha2::Sha256::digest(guess.as_bytes()));

    if hash.ends_with("0000") {
      return nonce;
    }

    nonce += 1;
  }
}
