use std::collections::HashMap;
use zgvariant::{Value, serialized::Context};

#[test]
fn struct_byte_array() {
    let ctxt = Context::new(zgvariant::LE, 0);
    // A non-fixed-sized, non-last struct field (the byte array here) must have some content: an
    // entirely empty one hits a pre-existing GVariant-format offset-table corner case shared with
    // zvariant (see https://github.com/z-galaxy/zbus/issues for the upstream behaviour), which
    // isn't what this test is about.
    let mut map = HashMap::new();
    map.insert("key".to_string(), Value::new("value"));
    let value: (Vec<u8>, HashMap<String, Value<'_>>) = (vec![1, 2, 3], map);
    let value = zgvariant::to_bytes(ctxt, &value).unwrap();
    #[cfg(feature = "serde_bytes")]
    let (bytes, map): (&serde_bytes::Bytes, HashMap<&str, Value<'_>>) = value
        .deserialize()
        .expect("Could not deserialize serde_bytes::Bytes in struct.")
        .0;
    #[cfg(not(feature = "serde_bytes"))]
    let (bytes, map): (&[u8], HashMap<&str, Value<'_>>) = value
        .deserialize()
        .expect("Could not deserialize u8 slice in struct")
        .0;

    assert_eq!(&bytes[..], [1u8, 2, 3]);
    assert_eq!(map["key"], Value::new("value"));
}
