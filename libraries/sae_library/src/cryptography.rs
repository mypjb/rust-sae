use std::string::ParseError;

pub mod hmac;
///MD5 Cryptography
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
pub struct Builder {
    /// <code>Cryptography</code> context
    context: Context,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            context: Default::default(),
        }
    }
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
    // pub fn add_cryptography<T>(&self) -> T where T:Default+Cryptography {
    //     let cryptography = T::default();
    //     cryptography.add_context(self.context.clone());
    //     return cryptography;
    // }
    pub fn add_cryptography<T: Cryptography + Default>(&self) -> Box<dyn Cryptography> {
        let t = T::default();
        let context = self.context.clone();
        let cryptography = <T as Cryptography>::add_context(Box::new(t), context);
        return cryptography;
    }

    /// Add public key
    pub fn add_public_key(mut self, public_key: String) -> Self {
        self.context.public_key = public_key;
        return self;
    }

    /// Add private key
    pub fn add_private_key(mut self, private_key: String) -> Self {
        self.context.private_key = private_key;
        return self;
    }
}

/// Cryptography interface
pub trait Cryptography {
    /// Set <code>Cryptography<code/> context
    fn add_context(self: Box<Self>, context: Context) -> Box<dyn Cryptography>;

    /// Build <code>Cryptography<code/>
    fn build(self: Box<Self>) -> Box<dyn Cryptography>;

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
