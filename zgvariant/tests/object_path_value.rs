use zgvariant::{LE, ObjectPath, Value};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn object_path_value() {
    let o = ObjectPath::try_from("/hello/world").unwrap();
    basic_type_test!(LE, o, 13, ObjectPath<'_>, 1);

    // As Value
    let v: Value<'_> = o.into();
    assert_eq!(v.value_signature(), "o");
    let encoded = value_test!(LE, v, 15);
    let v = encoded.deserialize::<Value<'_>>().unwrap().0;
    assert_eq!(
        v,
        Value::ObjectPath(ObjectPath::try_from("/hello/world").unwrap())
    );
}
