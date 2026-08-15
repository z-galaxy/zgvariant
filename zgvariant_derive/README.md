# zgvariant_derive

Derive macros for the [zgvariant] crate: `Type`, `Value`, `OwnedValue`, `SerializeDict` and
`DeserializeDict`, plus the `signature!` proc macro.

The `zgvariant` crate re-exports everything from here, so you normally don't need to depend on
this crate directly. Depend on it only if you need the macros without pulling in `zgvariant`
itself.

[zgvariant]: https://crates.io/crates/zgvariant
