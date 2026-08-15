use zgvariant::{serialized::Context, to_bytes_for_signature};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn enums() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum Unit {
        Variant1,
        Variant2,
        Variant3,
    }

    let ctxts_n_expected_lens = [
        (Context::new(zgvariant::BE, 0), 4usize),
        (Context::new(zgvariant::BE, 1), 7),
        (Context::new(zgvariant::BE, 2), 6),
        (Context::new(zgvariant::BE, 3), 5),
        (Context::new(zgvariant::BE, 4), 4),
    ];
    for (ctxt, expected_len) in ctxts_n_expected_lens {
        let encoded = to_bytes_for_signature(ctxt, "u", &Unit::Variant2).unwrap();
        assert_eq!(encoded.len(), expected_len);
        let decoded: Unit = encoded.deserialize_for_signature("u").unwrap().0;
        assert_eq!(decoded, Unit::Variant2);
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum NewType<'s> {
        Variant1(&'s str),
        Variant2(&'s str),
        Variant3(&'s str),
    }

    let ctxts_n_expected_lens = [
        (Context::new(zgvariant::BE, 0), 10usize),
        (Context::new(zgvariant::BE, 1), 13),
        (Context::new(zgvariant::BE, 2), 12),
        (Context::new(zgvariant::BE, 3), 11),
        (Context::new(zgvariant::BE, 4), 10),
    ];
    for (ctxt, expected_len) in ctxts_n_expected_lens {
        let encoded = to_bytes_for_signature(ctxt, "(us)", &NewType::Variant2("hello")).unwrap();
        assert_eq!(encoded.len(), expected_len);
        let decoded: NewType<'_> = encoded.deserialize_for_signature("(us)").unwrap().0;
        assert_eq!(decoded, NewType::Variant2("hello"));
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum Structs {
        Tuple(u8, u32),
        Struct { y: u8, t: u32 },
    }

    let ctxts_n_expected_lens = [
        (Context::new(zgvariant::BE, 0), 12usize),
        (Context::new(zgvariant::BE, 1), 15),
        (Context::new(zgvariant::BE, 2), 14),
        (Context::new(zgvariant::BE, 3), 13),
        (Context::new(zgvariant::BE, 4), 12),
    ];
    // TODO: Provide convenience API to create complex signatures
    let signature = "(u(yu))";
    for (ctxt, expected_len) in ctxts_n_expected_lens {
        let encoded = to_bytes_for_signature(ctxt, signature, &Structs::Tuple(42, 42)).unwrap();
        assert_eq!(encoded.len(), expected_len);
        let decoded: Structs = encoded.deserialize_for_signature(signature).unwrap().0;
        assert_eq!(decoded, Structs::Tuple(42, 42));

        let s = Structs::Struct { y: 42, t: 42 };
        let encoded = to_bytes_for_signature(ctxt, signature, &s).unwrap();
        assert_eq!(encoded.len(), expected_len);
        let decoded: Structs = encoded.deserialize_for_signature(signature).unwrap().0;
        assert_eq!(decoded, Structs::Struct { y: 42, t: 42 });
    }
}
