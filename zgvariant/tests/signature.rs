use zgvariant::{LE, Signature, Variant, signature};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn signature() {
    let sig: Signature = signature!("yys");

    // Structure will always add () around the signature if it's a struct.
    basic_type_test!(LE, sig, 6, Signature, 1);

    // As Variant
    let v: Variant<'_> = sig.into();
    assert_eq!(v.value_signature(), "g");
    let encoded = value_test!(LE, v, 8);
    let v = encoded.deserialize::<Variant<'_>>().unwrap().0;
    assert_eq!(v, Variant::Signature(signature!("yys")));
}
