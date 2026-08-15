use std::{
    borrow::Cow,
    ops::{Bound, Deref, Range, RangeBounds},
    sync::Arc,
};

use serde::{Deserialize, de::DeserializeSeed};

use crate::{
    DynamicDeserialize, DynamicType, Error, Result, Signature, Type, de::Deserializer,
    serialized::Context,
};

/// Represents serialized bytes in the GVariant format.
#[derive(Clone, Debug)]
pub struct Data<'bytes> {
    inner: Arc<Cow<'bytes, [u8]>>,
    context: Context,
    range: Range<usize>,
}

impl<'bytes> Data<'bytes> {
    /// Create a new `Data` instance.
    pub fn new<T>(bytes: T, context: Context) -> Self
    where
        T: Into<Cow<'bytes, [u8]>>,
    {
        let bytes = bytes.into();
        let range = Range {
            start: 0,
            end: bytes.len(),
        };
        Data {
            inner: Arc::new(bytes),
            context,
            range,
        }
    }

    /// The serialized bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.inner[self.range.start..self.range.end]
    }

    /// The encoding context.
    pub fn context(&self) -> Context {
        self.context
    }

    /// Returns a slice of `self` for the provided range.
    ///
    /// # Panics
    ///
    /// Requires that begin <= end and end <= self.len(), otherwise slicing will panic.
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Data<'bytes> {
        let len = self.range.end - self.range.start;
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        assert!(
            start <= end,
            "range start must not be greater than end: {start:?} > {end:?}",
        );
        assert!(end <= len, "range end out of bounds: {end:?} > {len:?}");

        let context = Context::new(self.context.endian(), self.context.position() + start);
        let range = Range {
            start: self.range.start + start,
            end: self.range.start + end,
        };

        Data {
            inner: self.inner.clone(),
            context,
            range,
        }
    }

    /// Deserialize `T` from `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zgvariant::LE;
    /// use zgvariant::to_bytes;
    /// use zgvariant::serialized::Context;
    ///
    /// let ctxt = Context::new(LE, 0);
    /// let encoded = to_bytes(ctxt, "hello world").unwrap();
    /// let decoded: &str = encoded.deserialize().unwrap().0;
    /// assert_eq!(decoded, "hello world");
    /// ```
    ///
    /// # Return value
    ///
    /// A tuple containing the deserialized value and the number of bytes parsed from `bytes`.
    pub fn deserialize<'d, T>(&'d self) -> Result<(T, usize)>
    where
        T: Deserialize<'d> + Type,
    {
        self.deserialize_for_signature(T::SIGNATURE)
    }

    /// Deserialize `T` from `self` with the given signature.
    ///
    /// Use this method instead of [`Data::deserialize`] if the value being deserialized does not
    /// implement [`Type`].
    ///
    /// # Examples
    ///
    /// While `Type` derive supports enums, for this example, let's supposed it doesn't and we don't
    /// want to manually implement `Type` trait either:
    ///
    /// ```rust
    /// use serde::{Deserialize, Serialize};
    /// use zgvariant::{
    ///     LE, to_bytes_for_signature, serialized::Context,
    ///     signature::{Signature, Fields},
    /// };
    ///
    /// let ctxt = Context::new(LE, 0);
    /// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    /// enum Unit {
    ///     Variant1,
    ///     Variant2,
    ///     Variant3,
    /// }
    ///
    /// let encoded = to_bytes_for_signature(ctxt, &Signature::U32, &Unit::Variant2).unwrap();
    /// assert_eq!(encoded.len(), 4);
    /// let decoded: Unit = encoded.deserialize_for_signature(&Signature::U32).unwrap().0;
    /// assert_eq!(decoded, Unit::Variant2);
    ///
    /// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    /// enum NewType<'s> {
    ///     Variant1(&'s str),
    ///     Variant2(&'s str),
    ///     Variant3(&'s str),
    /// }
    ///
    /// let signature = Signature::Structure(Fields::Static {
    ///     fields: &[&Signature::U32, &Signature::Str],
    /// });
    /// let encoded =
    ///     to_bytes_for_signature(ctxt, &signature, &NewType::Variant2("hello")).unwrap();
    /// assert_eq!(encoded.len(), 10);
    /// let decoded: NewType<'_> = encoded.deserialize_for_signature(&signature).unwrap().0;
    /// assert_eq!(decoded, NewType::Variant2("hello"));
    ///
    /// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    /// enum Structs {
    ///     Tuple(u8, u64),
    ///     Struct { y: u8, t: u64 },
    /// }
    ///
    /// let signature = Signature::Structure(Fields::Static {
    ///     fields: &[
    ///         &Signature::U32,
    ///         &Signature::Structure(Fields::Static {
    ///             fields: &[&Signature::U8, &Signature::U64],
    ///         }),
    ///     ],
    /// });
    /// let encoded = to_bytes_for_signature(ctxt, &signature, &Structs::Tuple(42, 42)).unwrap();
    /// assert_eq!(encoded.len(), 24);
    /// let decoded: Structs = encoded.deserialize_for_signature(&signature).unwrap().0;
    /// assert_eq!(decoded, Structs::Tuple(42, 42));
    ///
    /// let s = Structs::Struct { y: 42, t: 42 };
    /// let encoded = to_bytes_for_signature(ctxt, &signature, &s).unwrap();
    /// assert_eq!(encoded.len(), 24);
    /// let decoded: Structs = encoded.deserialize_for_signature(&signature).unwrap().0;
    /// assert_eq!(decoded, Structs::Struct { y: 42, t: 42 });
    /// ```
    ///
    /// # Return value
    ///
    /// A tuple containing the deserialized value and the number of bytes parsed from `bytes`.
    pub fn deserialize_for_signature<'d, S, T>(&'d self, signature: S) -> Result<(T, usize)>
    where
        T: Deserialize<'d>,
        S: TryInto<Signature>,
        S::Error: Into<Error>,
    {
        let signature = signature.try_into().map_err(Into::into)?;

        let mut de = Deserializer::new(self.bytes(), &signature, self.context)?;

        T::deserialize(&mut de).map(|t| (t, de.0.pos))
    }

    /// Deserialize `T` from `self`, with the given dynamic signature.
    ///
    /// # Return value
    ///
    /// A tuple containing the deserialized value and the number of bytes parsed from `bytes`.
    pub fn deserialize_for_dynamic_signature<'d, S, T>(&'d self, signature: S) -> Result<(T, usize)>
    where
        T: DynamicDeserialize<'d>,
        S: TryInto<Signature>,
        S::Error: Into<Error>,
    {
        let signature = signature.try_into().map_err(Into::into)?;
        let seed = T::deserializer_for_signature(&signature)?;

        self.deserialize_with_seed(seed)
    }

    /// Deserialize `T` from `self`, using the given seed.
    ///
    /// # Return value
    ///
    /// A tuple containing the deserialized value and the number of bytes parsed from `bytes`.
    pub fn deserialize_with_seed<'d, S>(&'d self, seed: S) -> Result<(S::Value, usize)>
    where
        S: DeserializeSeed<'d> + DynamicType,
    {
        let signature = S::signature(&seed);

        let mut de = Deserializer::new(self.bytes(), &signature, self.context)?;

        seed.deserialize(&mut de).map(|t| (t, de.0.pos))
    }
}

impl Deref for Data<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes()
    }
}

impl<T> AsRef<T> for Data<'_>
where
    T: ?Sized,
    for<'bytes> <Data<'bytes> as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}
