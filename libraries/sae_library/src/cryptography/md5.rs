use std::string::ParseError;

use crate::cryptography::{Builder, Context, Cryptography};
#[derive(Default)]
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

    fn add_context(self: Box<Self>, context: Context) -> Box<dyn Cryptography> {
        self
    }

    fn build(self: Box<Self>) -> Box<dyn Cryptography> {
        self
    }
}
///MD5 cryptography
pub trait MD5CryptographyBuilder {
    /// Build md5 cryptography
    /// # Examples
    /// ```ignore
    /// let builder = Builder::default();
    /// let md5_cryptography = builder.build_md5();
    /// md5_cryptography.encrypt(&plain_text);
    /// ```
    fn build_md5(&self) -> Box<dyn Cryptography>;
}

impl MD5CryptographyBuilder for Builder {
    /// Build md5 cryptography
    /// # Examples
    /// ```ignore
    /// let builder = Builder::default();
    /// let md5_cryptography = builder.build_md5();
    /// md5_cryptography.encrypt(&plain_text);
    /// ```
    fn build_md5(&self) -> Box<dyn Cryptography> {
        let cryptography= self.add_cryptography::<MD5Cryptography>();
        return  cryptography.build();
    }
}
