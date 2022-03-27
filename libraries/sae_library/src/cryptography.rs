use std::{default, string::ParseError};

pub mod md5;

#[derive(Clone, Default)]
pub struct Context {
    public_key: String,
    private_key: String,
}

#[derive(Default)]
pub struct Builder {
    context: Context
}

impl Builder {
    /// build <code>Cryptography</code> interface
    pub fn build(&self,cryptography:Box<dyn Cryptography>) -> Box<dyn Cryptography> {
        cryptography.add_context(self.context.clone());
        return cryptography;
    }
    /// add public key
    pub fn add_public_key(mut self, public_key: String) -> Builder {
        return Builder {
            context: self.context.clone()
        };
    }
    /// add private key
    pub fn add_private_key(mut self, public_key: String) -> Builder {
        return Builder {
            context: self.context
        };
    }
}

/// Cryptography interface
pub trait Cryptography {
    /// set <code>Cryptography<code/> context
    fn add_context(&self, context: Context);

    /// encrypt
    fn encrypt(&self, plain_text: &[u8]) -> Vec<u8> {
        panic!("encrypt algorithm has not been implemented");
    }
    
    ///decrypt
    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, ParseError> {
        panic!("decrypt algorithm has not been implemented");
    }
}
