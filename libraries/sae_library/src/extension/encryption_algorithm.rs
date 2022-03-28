use crate::cryptography::md5::*;
use crate::cryptography::{Builder};

///Encryption algorithm extensions
pub trait EncryptionAlgorithm {
    /// <strong>MD5</strong> algorithm return <strong>hash</strong> <code>String</code>
    fn md5_str(&self) -> String {
        panic!("md5 algorithm has not been implemented")
    }
    /// <strong>MD5</strong> algorithm return <strong>hash</strong> <code>[u8; 16]</code>
    fn md5_byte(&self) -> [u8; 16] {
        panic!("md5 algorithm has not been implemented")
    }
}

/// Use <strong>md5</strong> encryption <code>str</code>
impl EncryptionAlgorithm for str {
    /// # Examples
    /// ```ignore
    /// let str = "111111";
    ///
    /// //"96e79218965eb72c92a549dd5a330112"
    /// let hash = str.md5_str();
    /// ```
    fn md5_str(&self) -> String {
        self.as_bytes().md5_str()
    }
    /// # Examples
    /// ```ignore
    /// let str = "111111";
    /// // print [u8;16] : [xxx,xxx,xxx,xxx...]
    /// print!(str.md5_byte());
    /// ```
    fn md5_byte(&self) -> [u8; 16] {
        self.as_bytes().md5_byte()
    }
}

/// Use <strong>md5</strong> encryption <code>bytes</code>
impl EncryptionAlgorithm for [u8] {
    /// # Examples
    /// ```ignore
    /// let bytes = "111111".as_bytes();
    ///
    /// //"96e79218965eb72c92a549dd5a330112"
    /// let hash = bytes.md5_str();
    ///
    /// ```
    fn md5_str(&self) -> String {
        let bytes = self.md5_byte();

        let mut hash = String::new();

        for by in bytes {
            hash.push_str(format!("{:02x}", by).as_str());
        }

        return hash;
    }

    /// # Examples
    /// ```ignore
    /// let bytes = "111111".as_bytes();
    /// // print [u8;16] : [xxx,xxx,xxx,xxx...]
    /// print!(bytes.md5_byte());
    /// ```
    fn md5_byte(&self) -> [u8; 16] {
        let builder = Builder::default();
        let cryptography = builder.build(Box::new(MD5Cryptography {}));
        const ARRAY_LENGTH: usize = 16;

        let mut array = [0; ARRAY_LENGTH];

        let bytes = cryptography.encrypt(self);

        let bytes_length = bytes.len();

        if bytes_length == ARRAY_LENGTH {
            let mut i = 0;
            while i < ARRAY_LENGTH {
                array[i] = bytes[i];
                i += 1;
            }
        } else if bytes_length > 0 {
            print!(
                "md5 encryption failed,return byte lenght not {}, is {}",
                ARRAY_LENGTH, bytes_length
            );
        }

        return array;
    }
}
