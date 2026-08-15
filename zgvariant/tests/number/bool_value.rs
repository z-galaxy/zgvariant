use zgvariant::{LE, serialized::Context, to_bytes};

#[test]
fn bool_value() {
    let gvariant = basic_type_test!(LE, true, 1, bool, 1, Bool, 3);
    assert_eq!(*gvariant.bytes(), [1]);
}

#[test]
fn bool_maybe_array_value() {
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &vec![Some(true); 3]).unwrap();
    assert_eq!(encoded.bytes(), b"\x01\x01\x01\x01\x02\x03");
    let decoded: Vec<Option<bool>> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, vec![Some(true); 3]);
}
