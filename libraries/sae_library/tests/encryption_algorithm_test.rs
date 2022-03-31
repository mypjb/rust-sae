use sae_library::extension::*;
use sae_library::cryptography::*;
use sae_library::cryptography::hmac::*;
use std::{fmt::Write};

#[test]
fn md5_test() {
    let st = String::from("111111");

    assert_eq!(st.md5_str(), "96e79218965eb72c92a549dd5a330112");

    let mut hash = String::new();

    for byte in st.md5_byte() {
        write!(hash, "{:02x}", byte).unwrap();
    }

    assert_eq!(st.md5_str(), hash);
}

#[test]
fn hmac_test() {
    let key = uuid::Uuid::new_v4().to_string();

    let cryptography = Builder::default()
        .add_private_key(key.clone())
        .add_cryptography::<HMACCryptography>()
        .build();

    let plain_text = "Hello Rust my PJB!";

    let plain_bytes = plain_text.as_bytes();

    let cipher_bytes = cryptography.encrypt(plain_bytes);

    let key_bytes= key.as_bytes();

    let hamc_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA512, key_bytes);

    let tag = ring::hmac::sign(&hamc_key, plain_bytes);
    
    assert_eq!(tag.as_ref(),cipher_bytes.as_slice());
}
