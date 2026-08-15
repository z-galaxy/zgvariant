#[test]
#[cfg(unix)]
fn unit_fds() {
    use zgvariant::{BE, serialized::Context, to_bytes};

    let ctxt = Context::new(BE, 0);
    let encoded = to_bytes(ctxt, &()).unwrap();
    assert_eq!(encoded.len(), 1, "invalid encoding using `to_bytes`");
    let _: () = encoded
        .deserialize()
        .expect("invalid decoding using `from_slice`")
        .0;
}
