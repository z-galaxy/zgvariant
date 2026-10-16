//! The names that zgvariant 1.x used are still there, as aliases of the renamed items.
#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use zgvariant::{LE, OwnedValue, Type, Value, Variant, as_value, serialized::Context, to_bytes};

#[test]
fn value_and_owned_value_aliases() {
    let value: Value<'_> = Value::from(7u32);
    assert_eq!(value, Variant::U32(7));

    let owned: OwnedValue = value.try_to_owned().unwrap();
    assert_eq!(*owned, Variant::U32(7));
    assert_eq!(u32::try_from(owned).unwrap(), 7);
}

#[test]
fn value_value_makes_a_variant_holding_a_variant() {
    let nested = Value::Value(Box::new(Value::U8(42)));
    assert!(matches!(nested, Variant::Variant(inner) if *inner == Variant::U8(42)));
}

#[test]
fn value_and_owned_value_derives() {
    #[derive(Type, Value, OwnedValue, PartialEq, Debug)]
    struct Pair {
        x: i32,
        y: String,
    }

    let pair = || Pair {
        x: 42,
        y: "hello".to_string(),
    };
    let variant: Variant<'_> = Value::from(pair());
    assert_eq!(Pair::try_from(variant).unwrap(), pair());
    let owned = OwnedValue::try_from(pair()).unwrap();
    assert_eq!(Pair::try_from(owned).unwrap(), pair());
}

#[test]
fn as_value_module() {
    #[derive(Serialize, Deserialize, Type, PartialEq, Debug)]
    #[zgvariant(signature = "dict")]
    struct Props {
        #[serde(with = "as_value")]
        count: u32,
        #[serde(
            with = "zgvariant::as_value::optional",
            skip_serializing_if = "Option::is_none",
            default
        )]
        name: Option<String>,
    }

    let props = Props {
        count: 7,
        name: Some("seven".to_string()),
    };
    let encoded = to_bytes(Context::new(LE, 0), &props).unwrap();
    let decoded: Props = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, props);
}
