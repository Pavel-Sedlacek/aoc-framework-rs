use crate::md5::MessageDigest;

pub mod md5;

pub struct Crypto {
    md: MessageDigest
}

impl Crypto {
    pub fn new() -> Self {
        Crypto {
            md: MessageDigest::new()
        }
    }

    pub fn md5(&self, input: String) -> [u8; 16] {
        self.md.v5(input.as_bytes()).to_ne_bytes()
    }
}