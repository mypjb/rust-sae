use sae_library::*;
use std::fmt::Write;

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
