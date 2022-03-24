pub trait EncryptionAlgorithm {
    fn md5_str(&self) -> String {
        panic!("md5 algorithm has not been implemented")
    }

    fn md5_byte(&self) -> [u8; 16] {
        panic!("md5 algorithm has not been implemented")
    }
}

impl EncryptionAlgorithm for str {
    fn md5_str(&self) -> String {
        self.as_bytes().md5_str()
    }

    fn md5_byte(&self) -> [u8; 16] {
        self.as_bytes().md5_byte()
    }
}

impl EncryptionAlgorithm for [u8] {
    fn md5_str(&self) -> String {
        let bytes = self.md5_byte();

        let mut hash = String::new();

        for by in bytes {
            hash.push_str(format!("{:02x}", by).as_str());
        }

        return hash;
    }

    fn md5_byte(&self) -> [u8; 16] {
        let digest = md5::compute(self);
        return digest.into();
    }
}
