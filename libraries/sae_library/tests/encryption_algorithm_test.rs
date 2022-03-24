use sae_library::*;

#[test]
fn md5_test() {
    let st = String::from("111111");
    dbg!(st.md5_str());
    dbg!(st.md5_byte());
    let cr = st.md5_byte()[0];
    dbg!(format!("{:02x}",cr));
}
