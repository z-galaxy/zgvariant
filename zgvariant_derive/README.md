# zgvariant_derive

[![](https://docs.rs/zgvariant_derive/badge.svg)](https://docs.rs/zgvariant_derive/) [![](https://img.shields.io/crates/v/zgvariant_derive)](https://crates.io/crates/zgvariant_derive)

Derive macros for the [zgvariant] crate: `Type`, `Value`, `OwnedValue`, `SerializeDict` and
`DeserializeDict`, plus the `signature!` proc macro.

The `zgvariant` crate re-exports everything from here, so you normally don't need to depend on
this crate directly.

## Example code

```rust
use zgvariant::{serialized::Context, to_bytes, Type, LE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct<'s> {
    field1: u16,
    field2: i64,
    field3: &'s str,
}

assert_eq!(Struct::SIGNATURE, "(qxs)");
let s = Struct {
    field1: 42,
    field2: i64::max_value(),
    field3: "hello",
};
let ctxt = Context::new(LE, 0);
let encoded = to_bytes(ctxt, &s).unwrap();
let decoded: Struct = encoded.deserialize().unwrap().0;
assert_eq!(decoded, s);
```

See the [crate documentation] for the full set of macros and their attributes.

[zgvariant]: https://crates.io/crates/zgvariant
[crate documentation]: https://docs.rs/zgvariant_derive/latest/zgvariant_derive/
