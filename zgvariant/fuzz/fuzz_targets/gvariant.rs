#![no_main]
mod utils;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    utils::fuzz_for_context(data, zgvariant::serialized::Context::new(zgvariant::LE, 0));
    utils::fuzz_for_context(data, zgvariant::serialized::Context::new(zgvariant::BE, 0));
});
