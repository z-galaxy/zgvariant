use crate::Endian;

/// The encoding context to use with the [serialization] and [deserialization] API.
///
/// The encoding is dependent on the position of the encoding in the entire message and hence the
/// need to [specify] the byte position of the data being serialized or deserialized. Simply pass
/// `0` if serializing or deserializing to or from the beginning of message, or the preceding bytes
/// end on an 8-byte boundary.
///
/// # Examples
///
/// ```
/// use zgvariant::Endian;
/// use zgvariant::serialized::Context;
/// use zgvariant::to_bytes;
///
/// let pair = (1u32, 2u32);
/// let ctxt = Context::new(Endian::Little, 0);
/// let encoded = to_bytes(ctxt, &pair).unwrap();
///
/// // Let's decode the 2nd field only
/// let slice = encoded.slice(4..);
/// let decoded: u32 = slice.deserialize().unwrap().0;
/// assert_eq!(decoded, 2);
/// ```
///
/// [serialization]: zgvariant#functions
/// [deserialization]: zgvariant::serialized::Data::deserialize
/// [specify]: Context::new
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Context {
    position: usize,
    endian: Endian,
}

impl Context {
    /// Create a new encoding context.
    pub fn new(endian: Endian, position: usize) -> Self {
        Self { position, endian }
    }

    /// The [`Endian`] of this context.
    pub fn endian(self) -> Endian {
        self.endian
    }

    /// The byte position of the value to be encoded or decoded, in the entire message.
    pub fn position(self) -> usize {
        self.position
    }
}
