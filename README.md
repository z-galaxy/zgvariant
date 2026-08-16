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
