use std::collections::HashMap;
use zgvariant::{LE, Type, serialized::Context, to_bytes};

#[test]
fn struct_with_hashmap() {
    use serde::{Deserialize, Serialize};

    let mut hmap = HashMap::new();
    hmap.insert("key".into(), "value".into());

    #[derive(Type, Deserialize, Serialize, PartialEq, Debug)]
    struct Foo {
        hmap: HashMap<String, String>,
    }

    let foo = Foo { hmap };
    assert_eq!(Foo::SIGNATURE, "(a{ss})");

    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &(&foo, 1)).unwrap();
    // Deserialize the whole encoded top-level tuple, rather than just the leading `Foo` field in
    // isolation: GVariant's non-last variable-size struct members are only locatable via the
    // container's trailing offset table, so a type that doesn't match what was actually written
    // at the top level can't be decoded from a byte-prefix view the way it can in D-Bus.
    let (f, _): (Foo, i32) = encoded.deserialize().unwrap().0;
    assert_eq!(f, foo);
}
