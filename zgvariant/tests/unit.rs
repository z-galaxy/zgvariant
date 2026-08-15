use zgvariant::{BE, serialized::Context, to_bytes};

#[test]
fn unit() {
    let ctxt = Context::new(BE, 0);
    let encoded = to_bytes(ctxt, &()).unwrap();
    // Unlike D-Bus, GVariant has no true zero-byte encoding: the unit type is represented the
    // same way an empty fixed-size array is, as a single (unused) padding byte.
    assert_eq!(encoded.len(), 1, "invalid encoding using `to_bytes`");
    let _: () = encoded
        .deserialize()
        .expect("invalid decoding using `from_slice`")
        .0;
}
