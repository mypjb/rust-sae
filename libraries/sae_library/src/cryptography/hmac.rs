use std::ops::DerefMut;

use crate::Builder;
use crate::Cryptography;

use crate::Context;
use ring::{hmac, rand};

pub struct HMACCryptography {
    pub key: ring::hmac::Key,
    private_key: String,
}

impl Default for HMACCryptography {
    fn default() -> Self {
        let rng = rand::SystemRandom::new();
        let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng).unwrap();
        Self {
            key: key,
            private_key: Default::default(),
        }
    }
}

impl Cryptography for HMACCryptography {

    fn encrypt(&self, plain_text: &[u8]) -> Vec<u8> {
        let tag = hmac::sign(&self.key, plain_text);
        return tag.as_ref().to_vec();
    }

    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, std::string::ParseError> {
        panic!("decrypt algorithm has not been implemented");
    }

    fn add_context(mut self:Box<Self>, context: Context) -> Box<dyn Cryptography> {  
        self.private_key = context.private_key;
        self.key = ring::hmac::Key::new(hmac::HMAC_SHA512, self.private_key.as_bytes());
        self
    }

    fn build(self: Box<Self>) -> Box<dyn Cryptography> {
        self
    }
}

///MD5 cryptography
pub trait HMACCryptographyBuilder {
    /// Build md5 cryptography
    /// # Examples
    /// ```ignore
    /// let builder = Builder::default();
    /// let hmac_cryptography = builder.build_hamc();
    /// hmac_cryptography.encrypt(&plain_text);
    /// ```
    fn build_hamc(&self) -> Box<dyn Cryptography>;
}

impl HMACCryptographyBuilder for Builder {
    fn build_hamc(&self) -> Box<dyn Cryptography> {
        let cryptography= self.add_cryptography::<HMACCryptography>();
        return cryptography.build();
    }
}
