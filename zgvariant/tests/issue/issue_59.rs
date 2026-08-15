use zgvariant::{LE, serialized::Context, to_bytes};

#[test]
fn issue_59() {
    // Ensure we don't panic on deserializing a tuple of smaller than expected length. Unlike
    // D-Bus, GVariant's format is designed to always be decodable without erroring or panicking:
    // reading a 1-tuple's bytes as a 2-tuple yields a best-effort (if nonsensical) result rather
    // than a hard error.
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &("hello",)).unwrap();
    let (decoded, _): ((&str, &str), _) = encoded.deserialize().unwrap();
    assert_eq!(decoded, ("", "hello"));
}
