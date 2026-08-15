//! Round-trip tests for container sizes that straddle a framing-offset width boundary.
//!
//! GVariant sizes a framing offset from the length of the container *including* the offsets
//! themselves, so the width has to grow one byte earlier than a naive "does the body fit?" check
//! suggests. These tests pin that down for dict-entry key offsets, where getting it wrong makes
//! the encoder and the decoder disagree on how many bytes to read back.

use std::collections::HashMap;

use endi::NATIVE_ENDIAN;
use zgvariant::{serialized::Context, to_bytes};

#[test]
fn dict_entry_key_offset_widths() {
    // Sizes on both sides of the `u8` and `u16` framing-offset boundaries. The `u32` boundary
    // would need a 4 GiB allocation, so it's left to the arithmetic above.
    for body_len in [253, 254, 255, 256, 257, 65533, 65534, 65535, 65536] {
        round_trip_dict_entry(body_len);
    }
}

/// Round-trips a single-entry `a{ss}` dict whose entry body is exactly `body_len` bytes.
fn round_trip_dict_entry(body_len: usize) {
    // Both strings are nul-terminated and byte-aligned, so the entry body is exactly
    // `key.len() + value.len() + 2` with no padding in between.
    const KEY_LEN: usize = 3;
    let key = "k".repeat(KEY_LEN);
    let value = "v".repeat(body_len - KEY_LEN - 2);
    let map = HashMap::from([(key, value)]);

    let ctxt = Context::new(NATIVE_ENDIAN, 0);
    let encoded = to_bytes(ctxt, &map).unwrap();

    let (decoded, parsed) = encoded
        .deserialize::<HashMap<String, String>>()
        .unwrap_or_else(|e| panic!("decoding a {body_len}-byte dict-entry body: {e}"));
    assert_eq!(parsed, encoded.len());
    assert_eq!(
        decoded, map,
        "round-trip of a {body_len}-byte dict-entry body"
    );

    // The entry carries one key offset and the single-element array one element offset, each
    // sized from the container it terminates.
    let entry_len = body_len + offset_width(body_len, 1);
    assert_eq!(
        encoded.len(),
        entry_len + offset_width(entry_len, 1),
        "unexpected encoding of a {body_len}-byte dict-entry body"
    );
}

/// The width of each framing offset of a container holding `num_offsets` of them over
/// `content_len` bytes of content.
///
/// This is the GVariant rule spelled out independently of the implementation under test: the
/// narrowest width that can still address the container once its own offsets are counted in.
fn offset_width(content_len: usize, num_offsets: usize) -> usize {
    const MAXES: [(usize, usize); 4] = [
        (1, u8::MAX as usize),
        (2, u16::MAX as usize),
        (4, u32::MAX as usize),
        (8, usize::MAX),
    ];

    MAXES
        .into_iter()
        .find(|(width, max)| content_len + num_offsets * width <= *max)
        .map(|(width, _)| width)
        .expect("container too large to address")
}
