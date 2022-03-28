use std::{string::ParseError};

pub mod md5;

/// <code>Cryptography</code> context
#[derive(Clone, Default)]
pub struct Context {
    /// Public key
    pub public_key: String,
    /// Private key
    pub private_key: String,
}

/// <code>Cryptography</code> build
/// # Examples
/// ```ignore
/// let builder = Builder::default();
/// let plain_text:[u8] = [250;250];
/// let aes_cryptography = builder.add_public_key("xxx")
///                               .add_private_key("xxx")
///                               .build(Box::new(AESCryptography {}));
/// let cipher_text = aes_cryptography.encrypt(&plain_text);
/// 
/// assert_eq!(aes_cryptography.decrypt(&cipher_text),&plain_text);
/// //or
/// let md5_cryptography = builder.build(Box::new(MD5Cryptography {}));
/// md5_cryptography.encrypt(&plain_text);
/// ```
#[derive(Default)]
pub struct Builder {
    /// <code>Cryptography</code> context
    context: Context,
}

impl Builder {
    /// Build <code>Cryptography</code> interface
    /// # Examples
    /// ```ignore
    /// let builder = Builder::default();
    /// let plain_text:[u8] = [250;250];
    /// let aes_cryptography = builder.add_public_key("xxx")
    ///                               .add_private_key("xxx")
    ///                               .build(Box::new(AESCryptography {}));
    /// let cipher_text = aes_cryptography.encrypt(&plain_text);
    /// 
    /// assert_eq!(aes_cryptography.decrypt(&cipher_text),&plain_text);
    /// //or
    /// let md5_cryptography = builder.build(Box::new(MD5Cryptography {}));
    /// md5_cryptography.encrypt(&plain_text);
    /// ```
    pub fn build(&self, cryptography: Box<dyn Cryptography>) -> Box<dyn Cryptography> {
        cryptography.add_context(self.context.clone());
        return cryptography;
    }
    /// Add public key
    pub fn add_public_key(&self, public_key: String) -> Builder {
        let mut context = self.context.clone();
        context.public_key = public_key;
        return Builder { context: context };
    }
    /// Add private key
    pub fn add_private_key(self, private_key: String) -> Builder {
        let mut context = self.context.clone();
        context.private_key = private_key;
        return Builder { context: context };
    }
}

/// Cryptography interface
pub trait Cryptography {
    /// Set <code>Cryptography<code/> context
    fn add_context(&self, context: Context);

    /// Encrypt
    /// # Examples
    /// ```ignore
    /// let builder = Builder::default();
    /// let plain_text:[u8] = [250;250];
    /// let aes_cryptography = builder.add_public_key("xxx")
    ///                               .add_private_key("xxx")
    ///                               .build(Box::new(AESCryptography {}));
    /// 
    /// let cipher_text = aes_cryptography.encrypt(&plain_text);
    /// 
    /// assert_eq!(aes_cryptography.decrypt(&cipher_text),&plain_text);
    /// //or
    /// let md5_cryptography = builder.build(Box::new(MD5Cryptography {}));
    /// md5_cryptography.encrypt(&plain_text);
    /// ```
    fn encrypt(&self, plain_text: &[u8]) -> Vec<u8>;
    
    /// Decrypt
    /// # Examples
    /// ```ignore
    /// let builder = Builder::default();
    /// let plain_text:[u8] = [250;250];
    /// let aes_cryptography = builder.add_public_key("xxx")
    ///                               .add_private_key("xxx")
    ///                               .build(Box::new(AESCryptography {}));
    /// 
    /// let cipher_text = aes_cryptography.encrypt(&plain_text);
    /// 
    /// assert_eq!(aes_cryptography.decrypt(&cipher_text),&plain_text);
    /// //or
    /// let md5_cryptography = builder.build(Box::new(MD5Cryptography {}));
    /// md5_cryptography.encrypt(&plain_text);
    /// ```
    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, ParseError>;
}
