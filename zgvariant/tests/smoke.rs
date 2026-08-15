use zgvariant::{LE, Type, Value, serialized::Context, to_bytes};

#[test]
fn roundtrip_option_through_maybe() {
    let ctxt = Context::new(LE, 0);
    let value: Option<u32> = Some(42);
    assert_eq!(<Option<u32>>::SIGNATURE, "mu");
    let encoded = to_bytes(ctxt, &value).unwrap();
    let (decoded, _): (Option<u32>, _) = encoded.deserialize().unwrap();
    assert_eq!(decoded, Some(42));
}

#[test]
fn roundtrip_value_with_maybe() {
    let ctxt = Context::new(LE, 0);
    let value = Value::from(Some(7i64));
    let encoded = to_bytes(ctxt, &value).unwrap();
    let (decoded, _): (Value<'_>, _) = encoded.deserialize().unwrap();
    assert_eq!(decoded, value);
}
