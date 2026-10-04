//! Flags types defined with the [`bitflags`] crate need no special support from zgvariant.
//!
//! The recommended way to define such a type is bitflags' "impl form": declare the newtype struct
//! yourself, put whatever derives you need on it, and let the `bitflags!` macro generate the flags
//! API for it. The derives then see a plain newtype around an integer, so `Type`, `Value` and
//! `OwnedValue` delegate to that integer. So do serde's derived `Serialize` and `Deserialize`: they
//! (de)serialize a newtype struct as its single field, which zgvariant passes straight through to
//! the integer, so the wire format is the integer's. No `serde` feature of the `bitflags` crate is
//! needed.
//!
//! Only the macro-generated form of `bitflags!` needs that feature. Its serde support would use a
//! textual `"A | B"` form for human-readable formats, but zgvariant's (de)serializers are not
//! human-readable, so they get the plain integer there too.
//!
//! This is the recommended alternative to the `enumflags2` feature, which is kept for backwards
//! compatibility. This test is therefore not gated on that feature: it checks that the common
//! operations work with a flags type that is defined this way.

use std::{collections::HashMap, fmt};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use zgvariant::{BE, Basic, LE, OwnedValue, Signature, Type, Value, serialized::Context, to_bytes};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Value, OwnedValue)]
struct Flags(u32);

bitflags! {
    impl Flags: u32 {
        const ONE = 0x1;
        const TWO = 0x2;
        const FOUR = 0x4;
    }
}

// The `bitflags!` macro doesn't generate `Debug` in impl form, since the type is defined by the
// user. Print the flag names and any unknown bits in hex, as bitflags itself does when it generates
// `Debug`. That includes `0x0` for no flags at all: `to_writer` alone writes nothing in that case.
impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "{:#x}", self.bits());
        }
        bitflags::parser::to_writer(self, f)
    }
}

// The derives don't implement `Basic`, which converting a dictionary `Value` into a `HashMap`
// requires of the key type. It's needed for nothing else (see `value_conversions`).
impl Basic for Flags {
    const SIGNATURE_CHAR: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
}

#[test]
fn signature_is_that_of_the_underlying_integer() {
    assert_eq!(Flags::SIGNATURE, &Signature::U32);
    assert_eq!(Flags::SIGNATURE.to_string(), "u");
    assert_eq!(Flags::SIGNATURE, u32::SIGNATURE);

    // It composes with other types as the integer would.
    assert_eq!(<Vec<Flags>>::SIGNATURE.to_string(), "au");
    assert_eq!(<(Flags, String)>::SIGNATURE.to_string(), "(us)");
    assert_eq!(<HashMap<Flags, String>>::SIGNATURE.to_string(), "a{us}");
}

#[test]
fn wire_format_is_the_plain_integer() {
    let flags = Flags::ONE | Flags::FOUR;

    let encoded = to_bytes(Context::new(LE, 0), &flags).unwrap();
    assert_eq!(encoded.bytes(), &0x5u32.to_le_bytes()[..]);
    let (decoded, parsed): (Flags, _) = encoded.deserialize().unwrap();
    assert_eq!(decoded, flags);
    assert_eq!(parsed, encoded.len());

    let encoded = to_bytes(Context::new(BE, 0), &flags).unwrap();
    assert_eq!(encoded.bytes(), &0x5u32.to_be_bytes()[..]);
    let decoded: Flags = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, flags);

    // The integer encoding is interchangeable with the flags type's, in both directions.
    let encoded = to_bytes(Context::new(LE, 0), &0x2u32).unwrap();
    let decoded: Flags = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Flags::TWO);
    let encoded = to_bytes(Context::new(LE, 0), &Flags::TWO).unwrap();
    let decoded: u32 = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, 0x2);

    // Empty flags are valid too.
    let encoded = to_bytes(Context::new(LE, 0), &Flags::empty()).unwrap();
    assert_eq!(encoded.bytes(), &[0u8; 4][..]);
    let decoded: Flags = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Flags::empty());

    // GVariant's maybe type works too, and encodes the flags as the integer would.
    assert_eq!(<Option<Flags>>::SIGNATURE.to_string(), "mu");
    assert_eq!(<Option<Flags>>::SIGNATURE, <Option<u32>>::SIGNATURE);
    let some = Some(flags);
    let encoded = to_bytes(Context::new(LE, 0), &some).unwrap();
    let as_integer = to_bytes(Context::new(LE, 0), &Some(0x5u32)).unwrap();
    assert_eq!(encoded.bytes(), as_integer.bytes());
    assert_eq!(encoded.bytes(), &0x5u32.to_le_bytes()[..]);
    let decoded: Option<Flags> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, some);
    // `None` is its own thing, not empty flags.
    let encoded = to_bytes(Context::new(LE, 0), &None::<Flags>).unwrap();
    assert!(encoded.bytes().is_empty());
    let decoded: Option<Flags> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, None);
    let encoded = to_bytes(Context::new(LE, 0), &Some(Flags::empty())).unwrap();
    let decoded: Option<Flags> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Some(Flags::empty()));
}

#[test]
fn flags_nested_in_other_types() {
    #[derive(Serialize, Deserialize, Type, PartialEq, Debug)]
    struct Config {
        name: String,
        flags: Flags,
        history: Vec<Flags>,
        by_flags: HashMap<Flags, String>,
    }
    assert_eq!(Config::SIGNATURE.to_string(), "(suaua{us})");

    let config = Config {
        name: "test".to_string(),
        flags: Flags::ONE | Flags::TWO,
        history: vec![Flags::empty(), Flags::FOUR, Flags::all()],
        by_flags: HashMap::from([
            (Flags::ONE, "one".to_string()),
            (Flags::TWO, "two".to_string()),
        ]),
    };
    let encoded = to_bytes(Context::new(LE, 0), &config).unwrap();
    let decoded: Config = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, config);
}

#[test]
fn value_conversions() {
    let flags = Flags::TWO | Flags::FOUR;

    // Flags -> Value -> Flags.
    let value = Value::from(flags);
    assert_eq!(value, Value::U32(0x6));
    assert_eq!(value.value_signature().to_string(), "u");
    assert_eq!(Flags::try_from(value).unwrap(), flags);

    // A value of any other type is rejected.
    assert!(Flags::try_from(Value::from("not flags")).is_err());
    assert!(Flags::try_from(Value::from(0x6u8)).is_err());

    // The same through `OwnedValue`.
    let owned = OwnedValue::try_from(flags).unwrap();
    assert_eq!(*owned, Value::U32(0x6));
    assert_eq!(Flags::try_from(owned).unwrap(), flags);

    // A plain integer value is a valid flags value, going via `OwnedValue` as well.
    let owned: OwnedValue = Value::from(0x2u32).try_into().unwrap();
    assert_eq!(Flags::try_from(owned).unwrap(), Flags::TWO);
    let owned: OwnedValue = Value::from("not flags").try_into().unwrap();
    assert!(Flags::try_from(owned).is_err());

    // As the key of a dictionary value, which is why `Basic` is implemented above.
    let dict = HashMap::from([
        (Flags::ONE, "one".to_string()),
        (Flags::FOUR, "four".to_string()),
    ]);
    let value = Value::from(dict.clone());
    assert_eq!(value.value_signature().to_string(), "a{us}");
    assert_eq!(<HashMap<Flags, String>>::try_from(value).unwrap(), dict);

    // Flags inside a variant make a round trip through the wire. A `Value` that was deserialized
    // from a buffer borrows from it, whereas the `Value` derive converts from `Value<'static>` for
    // a type without lifetime parameters. So convert it to an owned value first.
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &Value::from(flags)).unwrap();
    let decoded: Value<'_> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded.value_signature().to_string(), "u");
    let owned = decoded.try_into_owned().unwrap();
    assert_eq!(Flags::try_from(owned).unwrap(), flags);
}

/// Unlike with `enumflags2`, whose `BitFlags` type rejects bits that don't belong to any flag, a
/// flags type from `bitflags` retains such bits. That keeps a service forward compatible with
/// peers that start using flags it doesn't know about yet.
#[test]
fn unknown_bits_are_retained() {
    let unknown = 0x8000_0001u32;
    let ctxt = Context::new(LE, 0);

    // The constructor that only accepts known bits would refuse these bits...
    assert_eq!(Flags::from_bits(unknown), None);

    // ... but deserializing, as well as converting from a `Value`, keeps them.
    let encoded = to_bytes(ctxt, &unknown).unwrap();
    let decoded: Flags = encoded.deserialize().unwrap().0;
    assert_eq!(decoded.bits(), unknown);
    assert!(decoded.contains(Flags::ONE));
    assert!(!decoded.contains(Flags::TWO));

    let from_value = Flags::try_from(Value::from(unknown)).unwrap();
    assert_eq!(from_value, decoded);

    let owned: OwnedValue = Value::from(unknown).try_into().unwrap();
    assert_eq!(Flags::try_from(owned).unwrap(), decoded);

    // They survive being serialized again, too.
    let reencoded = to_bytes(ctxt, &decoded).unwrap();
    assert_eq!(reencoded.bytes(), encoded.bytes());
}

#[test]
fn debug_prints_flag_names() {
    assert_eq!(format!("{:?}", Flags::ONE | Flags::FOUR), "ONE | FOUR");
    assert_eq!(format!("{:?}", Flags::empty()), "0x0");
    assert_eq!(format!("{:?}", Flags::all()), "ONE | TWO | FOUR");
    assert_eq!(
        format!("{:?}", Flags::from_bits_retain(0x8000_0002)),
        "TWO | 0x80000000"
    );
}
