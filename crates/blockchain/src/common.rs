use base64::engine::general_purpose;
use sha2::{Digest, Sha256};
use base64::Engine;

fn add_seed_to_suffix(s: &str, seed: &str) -> String {
    format!("{}{}", s, seed)
}

fn repeat10(s: &str) -> String {
    format!("{}", s.repeat(10))
}

fn repeat20(s: &str) -> String {
    format!("{}", s.repeat(20))
}

fn md5(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    format!("{:x}", digest)
}
fn base64(s: &str) -> String {
    let mut buf = String::new();
    general_purpose::STANDARD. encode_string(s.as_bytes(), &mut buf);
    format!("{}", buf)
}
pub fn sha256(s: &str) -> String {
    let data = s.as_bytes();
    let mut hasher = Sha256::new();

    // Write input data
    hasher.update(data);

    // Read hash digest
    let result = hasher.finalize();

    // Print hash in hexadecimal format
    format!("{:x}", result)
}
