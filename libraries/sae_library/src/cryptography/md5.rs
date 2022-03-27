use std::string::ParseError;

use crate::cryptography::{Builder, Context, Cryptography};
pub struct MD5Cryptography;
///MD5 algorithm
impl Cryptography for MD5Cryptography {
    fn encrypt(&self, plain_text: &[u8]) -> Vec<u8> {
        let digest = md5::compute(plain_text);
        return digest.to_vec();
    }

    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, ParseError> {
        panic!("decrypt algorithm has not been implemented");
    }
    fn add_context(&self, _: Context) {
        print!("the algorithm not context");
    }
}

pub trait MD5CryptographyBuilder {
    fn build_md5(&self) -> Box<dyn Cryptography>;
}

impl MD5CryptographyBuilder for  Builder{
    fn build_md5(&self) -> Box<dyn Cryptography> {
        self.build(Box::new(MD5Cryptography))
    }
}
