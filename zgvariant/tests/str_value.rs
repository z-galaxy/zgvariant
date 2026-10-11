use zgvariant::{LE, Variant, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn str_value() {
    let string = String::from("hello world");
    basic_type_test!(LE, string, 12, String, 1);

    let string = "hello world";
    basic_type_test!(LE, string, 12, &str, 1);

    // As Variant
    let v: Variant<'_> = string.into();
    assert_eq!(v.value_signature(), "s");
    assert_eq!(v, Variant::new("hello world"));
    value_test!(LE, v, 14);

    let v: String = v.try_into().unwrap();
    assert_eq!(v, "hello world");

    // Characters are treated as strings
    basic_type_test!(LE, 'c', 2, char, 1);

    // As Variant
    let v: Variant<'_> = "c".into();
    assert_eq!(v.value_signature(), "s");
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 4);
    let (v, _) = encoded.deserialize::<Variant<'_>>().unwrap();
    assert_eq!(v, Variant::new("c"));
}
