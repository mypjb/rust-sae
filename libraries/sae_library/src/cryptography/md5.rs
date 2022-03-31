use std::string::ParseError;
use crate::cryptography::{Context, Cryptography};

/// MD5 algorithm
#[derive(Default)]
pub struct MD5Cryptography;

impl Cryptography for MD5Cryptography {
    fn encrypt(&self, plain_text: &[u8]) -> Vec<u8> {
        let digest = md5::compute(plain_text);
        return digest.to_vec();
    }

    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, ParseError> {
        panic!("decrypt algorithm has not been implemented");
    }

    fn add_context(self: Box<Self>, context: Context) -> Box<dyn Cryptography> {
        self
    }

    fn build(self: Box<Self>) -> Box<dyn Cryptography> {
        self
    }
}
