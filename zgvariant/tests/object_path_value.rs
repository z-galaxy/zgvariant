use zgvariant::{LE, ObjectPath, Variant};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn object_path_value() {
    let o = ObjectPath::try_from("/hello/world").unwrap();
    basic_type_test!(LE, o, 13, ObjectPath<'_>, 1);

    // As Variant
    let v: Variant<'_> = o.into();
    assert_eq!(v.value_signature(), "o");
    let encoded = value_test!(LE, v, 15);
    let v = encoded.deserialize::<Variant<'_>>().unwrap().0;
    assert_eq!(
        v,
        Variant::ObjectPath(ObjectPath::try_from("/hello/world").unwrap())
    );
}
