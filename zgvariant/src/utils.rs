/// The prefix of ARRAY type signature, as a character. Provided for manual signature creation.
pub const ARRAY_SIGNATURE_CHAR: char = 'a';
/// The prefix of ARRAY type signature, as a string. Provided for manual signature creation.
pub const ARRAY_SIGNATURE_STR: &str = "a";
/// The opening character of STRUCT type signature. Provided for manual signature creation.
pub const STRUCT_SIG_START_CHAR: char = '(';
/// The closing character of STRUCT type signature. Provided for manual signature creation.
pub const STRUCT_SIG_END_CHAR: char = ')';
/// The opening character of STRUCT type signature, as a string. Provided for manual signature
/// creation.
pub const STRUCT_SIG_START_STR: &str = "(";
/// The closing character of STRUCT type signature, as a string. Provided for manual signature
/// creation.
pub const STRUCT_SIG_END_STR: &str = ")";
/// The opening character of DICT_ENTRY type signature. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_START_CHAR: char = '{';
/// The closing character of DICT_ENTRY type signature. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_END_CHAR: char = '}';
/// The opening character of DICT_ENTRY type signature, as a string. Provided for manual signature
/// creation.
pub const DICT_ENTRY_SIG_START_STR: &str = "{";
/// The closing character of DICT_ENTRY type signature, as a string. Provided for manual signature
/// creation.
pub const DICT_ENTRY_SIG_END_STR: &str = "}";
/// The VARIANT type signature. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_CHAR: char = 'v';
/// The VARIANT type signature, as a string. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_STR: &str = "v";
/// The prefix of MAYBE type signature, as a character. Provided for manual signature creation.
pub const MAYBE_SIGNATURE_CHAR: char = 'm';
/// The prefix of MAYBE type signature, as a string. Provided for manual signature creation.
pub const MAYBE_SIGNATURE_STR: &str = "m";
pub(crate) const VARIANT_ALIGNMENT_GVARIANT: usize = 8;

/// Calculates the padding needed to align `value` to the next multiple of `align`.
///
/// # Parameters
/// - `value`: The value to align.
/// - `align`: The alignment boundary. Must be a positive power of two.
///
/// # Panics
/// Panics if `align` is not a positive power of two.
// Public only for tests.
#[doc(hidden)]
pub fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    assert!(
        align > 0 && align.is_power_of_two(),
        "`align` must be a positive power of two"
    );
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

/// Slice the given slice of bytes safely and return an error if the slice is too small.
pub(crate) fn subslice<I, T>(input: &[T], index: I) -> crate::Result<&I::Output>
where
    I: std::slice::SliceIndex<[T]>,
{
    input.get(index).ok_or(crate::Error::OutOfBounds)
}
