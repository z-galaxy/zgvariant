# zgvariant

[![](https://docs.rs/zgvariant/badge.svg)](https://docs.rs/zgvariant/) [![](https://img.shields.io/crates/v/zgvariant)](https://crates.io/crates/zgvariant) [![CI Pipeline Status](https://github.com/z-galaxy/zgvariant/actions/workflows/rust.yml/badge.svg)](https://github.com/z-galaxy/zgvariant/actions/workflows/rust.yml)

This crate provides a [serde]-based API for encoding and decoding data to and from the
[GVariant] binary format. It started life as the `gvariant` cargo feature of [zvariant] (part of
the [zbus] project) and was later split out into its own crate, so that projects that only need
GVariant don't have to pull in zvariant's D-Bus-specific code, and vice versa.

If you're already familiar with zvariant, zgvariant should feel immediately familiar: the
serialization API is essentially unchanged, just pared down to a single wire format. See
[Migrating from zvariant's `gvariant` feature](#migrating-from-zvariants-gvariant-feature) below
if you're switching an existing project over.

If you're not familiar with [serde] itself, you may want to read its [tutorial] first.

## Example code

```rust
use zgvariant::{serialized::Context, to_bytes, Type, LE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
    label: String,
}

assert_eq!(Point::SIGNATURE, "(iis)");

let point = Point {
    x: 1,
    y: 2,
    label: "home".to_string(),
};
let ctxt = Context::new(LE, 0);
let encoded = to_bytes(ctxt, &point).unwrap();
let decoded: Point = encoded.deserialize().unwrap().0;
assert_eq!(decoded, point);
```

Have a look at the [`Type`], [`Value`] and [`OwnedValue`] documentation for more on GVariant's
type system and the generic `Value` container, and at [zgvariant_derive] for the full set of
derive macros (`Type`, `Value`, `OwnedValue`, `SerializeDict`, `DeserializeDict`) and the
`signature!` macro.

## Optional features

| Feature | Description |
| --- | --- |
| `arrayvec` | Implement `Type` for [`arrayvec::ArrayVec`] and [`arrayvec::ArrayString`] |
| `camino` | Implement `Type` for [`camino::Utf8Path`] and [`camino::Utf8PathBuf`] |
| `chrono` | Implement `Type` for various [`chrono`] date/time types |
| `enumflags2` | Implement `Type` for [`enumflags2::BitFlags`]`<F>` |
| `heapless` | Implement `Type` for [`heapless::Vec`] and [`heapless::String`] |
| `serde_bytes` | Implement `Type` for [`serde_bytes::Bytes`] and [`serde_bytes::ByteBuf`] |
| `time` | Implement `Type` for various [`time`] date/time types |
| `url` | Implement `Type` for [`url::Url`] |
| `uuid` | Implement `Type` for [`uuid::Uuid`] |
| `ostree-tests` | Enable the test that deserializes a real-world flatpak/ostree summary file |

## Flags types

For bit flags, we recommend the [`bitflags`] crate's "impl form". It needs no support from zgvariant
and no `serde` feature of `bitflags`: declare the newtype yourself with the derives you need on it,
and let the `bitflags!` macro generate the flags API for it. The derives treat the newtype as its
integer, so both the signature and the encoding are those of the integer:

```rust
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use zgvariant::{serialized::Context, to_bytes, Basic, OwnedValue, Type, Value, LE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Value, OwnedValue)]
struct Permissions(u32);

bitflags! {
    impl Permissions: u32 {
        const READ = 0x1;
        const WRITE = 0x2;
    }
}

// Only needed to use `Permissions` as a dictionary key when converting from a `Value`.
impl Basic for Permissions {
    const SIGNATURE_CHAR: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
}

assert_eq!(Permissions::SIGNATURE, "u");

let permissions = Permissions::READ | Permissions::WRITE;
let encoded = to_bytes(Context::new(LE, 0), &permissions).unwrap();
assert_eq!(encoded.bytes(), 3u32.to_le_bytes());
let decoded: Permissions = encoded.deserialize().unwrap().0;
assert_eq!(decoded, permissions);

let value = Value::from(permissions);
assert_eq!(Permissions::try_from(value).unwrap(), permissions);
```

Deserializing retains the bits that don't belong to any named flag (just like
`from_bits_retain`), so your code stays compatible with peers that start using flags it doesn't
know about yet. In this form, `bitflags!` doesn't generate `Debug` for you: the derive above prints
the raw integer (`Permissions(3)`), whereas implementing it with `bitflags::parser::to_writer`
prints the flag names. That function writes nothing for a value with no flags set, so print `0x0`
yourself in that case if you'd rather not get an empty string.

Converting a `Value` that borrows from a buffer (for example one you deserialized) into such a type
requires calling `try_into_owned()` on it first: for a type without lifetime parameters, the `Value`
derive converts only from `Value<'static>`, whereas the `enumflags2` feature accepts any lifetime.
Using the type as a dictionary key in conversions from `Value` requires `Basic`, which the derives
don't provide, hence the `impl Basic` above.

The macro-generated form (`bitflags! { struct Permissions: u32 { ... } }`) works too, as long as you
enable the `serde` feature of `bitflags` and add `#[zgvariant(signature = "u")]` to the type, but it
can't derive `Value` and `OwnedValue`. Since zgvariant's (de)serializers are not human-readable,
that form gets the plain integer rather than the `"READ | WRITE"` string that `bitflags` uses for
human-readable formats.

The `enumflags2` feature remains for compatibility with existing code that uses
[`enumflags2::BitFlags`]. Unlike the above, it rejects bits that don't belong to any flag.

## Migrating from zvariant's `gvariant` feature

| zvariant                            | zgvariant                                    |
| ----------------------------------- | -------------------------------------------- |
| `zvariant` with `gvariant` feature  | `zgvariant` (no feature needed)              |
| `zvariant::...` imports             | `zgvariant::...`                             |
| `Context::new_gvariant(endian, n)`  | `Context::new(endian, n)`                    |
| `serialized::Format` dispatch       | gone — there is only one format              |
| `option-as-array` feature           | gone — `Option<T>` is always a GVariant' `maybe` type|
| `Value::Fd` / fd passing            | not supported (D-Bus wire concept)           |
| `#[zvariant(...)]` derive attrs     | still accepted; `#[zgvariant(...)]` preferred|
| `SerializeValue`/`DeserializeValue` | gone — `as_value::{Serialize, Deserialize}`  |

`Signature` is the same type in both crates (re-exported from `zvariant_utils`), so signatures
can be passed between zvariant- and zgvariant-using code freely.

Note: a type cannot derive both zvariant's and zgvariant's `Type` in the same scope without
renaming one import, since the derive macros share their names.

## License

MIT license, see [LICENSE].

[serde]: https://crates.io/crates/serde
[GVariant]: https://developer.gnome.org/documentation/specifications/gvariant-specification-1.0.html
[zvariant]: https://crates.io/crates/zvariant
[zbus]: https://github.com/z-galaxy/zbus
[tutorial]: https://serde.rs/
[`Type`]: https://docs.rs/zgvariant/latest/zgvariant/trait.Type.html
[`Value`]: https://docs.rs/zgvariant/latest/zgvariant/enum.Value.html
[`OwnedValue`]: https://docs.rs/zgvariant/latest/zgvariant/struct.OwnedValue.html
[zgvariant_derive]: https://docs.rs/zgvariant_derive/latest/zgvariant_derive/
[`arrayvec::ArrayVec`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html
[`arrayvec::ArrayString`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayString.html
[`bitflags`]: https://docs.rs/bitflags/latest/bitflags/
[`camino::Utf8Path`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html
[`camino::Utf8PathBuf`]: https://docs.rs/camino/latest/camino/struct.Utf8PathBuf.html
[`chrono`]: https://docs.rs/chrono/latest/chrono/
[`enumflags2::BitFlags`]: https://docs.rs/enumflags2/latest/enumflags2/struct.BitFlags.html
[`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html
[`heapless::String`]: https://docs.rs/heapless/latest/heapless/struct.String.html
[`serde_bytes::Bytes`]: https://docs.rs/serde_bytes/latest/serde_bytes/struct.Bytes.html
[`serde_bytes::ByteBuf`]: https://docs.rs/serde_bytes/latest/serde_bytes/struct.ByteBuf.html
[`time`]: https://docs.rs/time/latest/time/
[`url::Url`]: https://docs.rs/url/latest/url/struct.Url.html
[`uuid::Uuid`]: https://docs.rs/uuid/latest/uuid/struct.Uuid.html
[LICENSE]: https://github.com/z-galaxy/zgvariant/blob/main/LICENSE
