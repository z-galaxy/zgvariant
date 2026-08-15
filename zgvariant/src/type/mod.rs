mod dynamic;
pub use dynamic::{DynamicDeserialize, DynamicType};
#[cfg(feature = "serde_bytes")]
mod bytes;
#[cfg(feature = "enumflags2")]
mod enumflags2;
mod libstd;
mod net;
mod paths;
mod time;
#[cfg(feature = "uuid")]
mod uuid;

use crate::Signature;

/// Trait implemented by all serializable types.
///
/// This very simple trait provides the signature for the implementing type. Since the [GVariant
/// type system] relies on these signatures, our serialization and deserialization API requires
/// this trait in addition to [`trait@serde::Serialize`] and [`serde::de::Deserialize`],
/// respectively.
///
/// Implementation is provided for all the [basic types](crate::Basic) and blanket implementations
/// for common container types, such as, arrays, slices, tuples, [`Vec`] and
/// [`std::collections::HashMap`]. For easy implementation for custom types, use `Type` derive macro
/// from [zgvariant_derive] crate.
///
/// If your type's signature cannot be determined statically, you should implement the
/// [`DynamicType`] trait instead, which is otherwise automatically implemented if you implement
/// this trait.
///
/// [GVariant type system]: https://developer.gnome.org/glib/stable/glib-GVariant.html
/// [zgvariant_derive]: https://docs.rs/zgvariant_derive/latest/zgvariant_derive/
pub trait Type {
    /// The signature for the implementing type, in parsed format.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use zgvariant::{Type, signature::{Child, Signature}};
    ///
    /// assert_eq!(u32::SIGNATURE, &Signature::U32);
    /// assert_eq!(String::SIGNATURE, &Signature::Str);
    /// assert_eq!(
    ///     <(u32, &str, u64)>::SIGNATURE,
    ///     &Signature::static_structure(&[&Signature::U32, &Signature::Str, &Signature::U64]),
    /// );
    /// assert_eq!(
    ///     <(u32, &str, &[u64])>::SIGNATURE,
    ///     &Signature::static_structure(&[
    ///         &Signature::U32,
    ///         &Signature::Str,
    ///         &Signature::Array(Child::Static { child: &Signature::U64 }),
    ///     ]),
    /// );
    /// assert_eq!(
    ///     <HashMap<u8, &str>>::SIGNATURE,
    ///     &Signature::static_dict(&Signature::U8, &Signature::Str),
    /// );
    /// ```
    const SIGNATURE: &'static Signature;
}

/// Implements the [`Type`] trait by delegating the signature to a simpler type (usually a tuple).
///
/// Example:
/// ```no_compile
/// impl_type_with_repr! {
///    // Duration is serialized as a (u64, u32) pair.
///    Duration => (u64, u32) {
///        // The macro auto-generates tests for us,
///        // so we need to provide a test name.
///        duration {
///            // Sample values used to test serialize compatibility.
///            samples = [Duration::ZERO, Duration::MAX],
///            // Converts our type into the simpler "repr" type.
///            repr(d) = (d.as_secs(), d.subsec_nanos()),
///        }
///    }
/// }
/// ```
// The `$test_mod`/`samples`/`repr` fragments are accepted but currently unused: round-trip
// testing a repr conversion needs `to_bytes`/`Context`, which live in the not-yet-implemented
// codec. Keeping the fragments in the matcher means invocation sites won't need to change once
// the codec lands and the test generation is added back.
#[macro_export]
macro_rules! impl_type_with_repr {
    ($($ty:ident)::+ $(<$typaram:ident $(: $($tbound:ident)::+)?>)? => $repr:ty {
        $test_mod:ident $(<$($typaram_sample:ident = $typaram_sample_value:ty),*>)? {
            $(signature = $signature:literal,)?
            samples = $samples:expr,
            repr($sample_ident:ident) = $into_repr:expr,
        }
    }) => {
        impl $(<$typaram $(: $($tbound)::+)?>)? $crate::Type for $($ty)::+ $(<$typaram>)? {
            const SIGNATURE: &'static $crate::Signature = <$repr>::SIGNATURE;
        }
    };
}

#[macro_export]
#[allow(unused)]
macro_rules! static_str_type {
    ($ty:ty) => {
        impl Type for $ty {
            const SIGNATURE: &'static Signature = &Signature::Str;
        }
    };
}
