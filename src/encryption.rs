use sha2::{Sha256, Digest};

pub fn encrypt(msg: &str) -> String {

    let mut hasher = Sha256::new();

    hasher.update(msg);

    format!("{:x}", hasher.finalize())
}
