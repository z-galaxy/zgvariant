# Fuzz targets for zgvariant

[Fuzzing](https://en.wikipedia.org/wiki/Fuzzing) is a way to test software by feeding it random
inputs to make sure it doesn't crash. This directory contains a target to test zgvariant using
[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

Run `cargo install cargo-fuzz` to install the fuzzer, then run `cargo +nightly fuzz run gvariant`
from the `zgvariant` directory to fuzz the GVariant deserializer.
