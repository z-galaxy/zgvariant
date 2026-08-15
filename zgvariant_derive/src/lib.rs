#![deny(rust_2018_idioms)]
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use zvariant_utils::derive::{self, Config};

fn config() -> Config {
    Config {
        attr_lists: &["zgvariant", "zvariant"],
        default_path: quote! { ::zgvariant },
    }
}

/// Derive macro to add [`Type`] implementation to structs and enums.
///
/// # Examples
///
/// For structs it works just like serde's [`Serialize`] and [`Deserialize`] macros:
///
/// ```
/// use zgvariant::{serialized::Context, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// struct Struct<'s> {
///     field1: u16,
///     field2: i64,
///     field3: &'s str,
/// }
///
/// assert_eq!(Struct::SIGNATURE, "(qxs)");
/// let s = Struct {
///     field1: 42,
///     field2: i64::max_value(),
///     field3: "hello",
/// };
/// let ctxt = Context::new(LE, 0);
/// let encoded = to_bytes(ctxt, &s).unwrap();
/// let decoded: Struct = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, s);
/// ```
///
/// Same with enum, except that all variants of the enum must have the same number and types of
/// fields (if any). If you want the encoding size of the (unit-type) enum to be dictated by
/// `repr` attribute (like in the example below), you'll also need the [serde_repr] crate.
///
/// ```
/// use zgvariant::{serialized::Context, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
/// use serde_repr::{Deserialize_repr, Serialize_repr};
///
/// #[repr(u8)]
/// #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
/// enum Enum {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(Enum::SIGNATURE, u8::SIGNATURE);
/// let ctxt = Context::new(LE, 0);
/// let encoded = to_bytes(ctxt, &Enum::Variant2).unwrap();
/// let decoded: Enum = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, Enum::Variant2);
///
/// #[repr(i64)]
/// #[derive(Deserialize_repr, Serialize_repr, Type)]
/// enum Enum2 {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(Enum2::SIGNATURE, i64::SIGNATURE);
///
/// // w/o repr attribute, u32 representation is chosen
/// #[derive(Deserialize, Serialize, Type)]
/// enum NoReprEnum {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(NoReprEnum::SIGNATURE, u32::SIGNATURE);
///
/// // Not-unit enums are represented as a structure, with the first field being a u32 denoting
/// // the variant and the second as the actual value.
/// #[derive(Deserialize, Serialize, Type)]
/// enum NewType {
///     Variant1(f64),
///     Variant2(f64),
/// }
/// assert_eq!(NewType::SIGNATURE, "(ud)");
///
/// #[derive(Deserialize, Serialize, Type)]
/// enum StructFields {
///     Variant1(u16, i64, &'static str),
///     Variant2 { field1: u16, field2: i64, field3: &'static str },
/// }
/// assert_eq!(StructFields::SIGNATURE, "(u(qxs))");
/// ```
///
/// # Custom signatures
///
/// A `#[zgvariant(signature = "...")]` attribute lets you hardcode the signature instead of
/// deriving it from the fields. A common use is encoding a struct as a dictionary (signature
/// `a{sv}`), for which `dict` is provided as a convenient alias:
///
/// ```
/// use zgvariant::{serialized::Context, as_value, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// // `#[zgvariant(signature = "a{sv}")]` would be the same.
/// #[zgvariant(signature = "dict")]
/// struct Struct {
///     #[serde(with = "as_value")]
///     field1: u16,
///     #[serde(with = "as_value")]
///     field2: i64,
///     #[serde(with = "as_value")]
///     field3: String,
/// }
///
/// assert_eq!(Struct::SIGNATURE, "a{sv}");
/// let s = Struct {
///     field1: 42,
///     field2: i64::max_value(),
///     field3: "hello".to_string(),
/// };
/// let ctxt = Context::new(LE, 0);
/// let encoded = to_bytes(ctxt, &s).unwrap();
/// let decoded: Struct = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, s);
/// ```
///
/// Another common use for custom signatures is (de)serializing a unit-only enum as a string
/// rather than as an integer:
///
/// ```
/// use zgvariant::{serialized::Context, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// #[zgvariant(signature = "s")]
/// enum StrEnum {
///     Variant1,
///     Variant2,
///     Variant3,
/// }
///
/// assert_eq!(StrEnum::SIGNATURE, "s");
/// let ctxt = Context::new(LE, 0);
/// let encoded = to_bytes(ctxt, &StrEnum::Variant2).unwrap();
/// let decoded: StrEnum = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, StrEnum::Variant2);
/// ```
///
/// # Custom crate path
///
/// If you've renamed `zgvariant` in your `Cargo.toml` or are using it through a re-export, you
/// can specify the crate path with `#[zgvariant(crate = "...")]`:
///
/// ```
/// use zgvariant::Type;
///
/// #[derive(Type)]
/// #[zgvariant(crate = "zgvariant")]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// # Migrating from `zvariant`
///
/// The `#[zvariant(...)]` helper attribute is also accepted, as an alias for
/// `#[zgvariant(...)]`, so types deriving `zvariant::Type` today keep compiling unchanged once
/// switched over to deriving `zgvariant::Type`.
///
/// [`Type`]: https://docs.rs/zgvariant/latest/zgvariant/trait.Type.html
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [serde_repr]: https://crates.io/crates/serde_repr
#[proc_macro_derive(Type, attributes(zgvariant, zvariant))]
pub fn type_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    derive::expand_type_derive(ast, &config())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Adds a [`Serialize`] implementation to structs to be serialized as a dictionary type.
///
/// The dictionary type is determined by the `signature` attribute. The default is `a{sv}`
/// (string keys, variant values), but nested forms like `a{sa{sv}}` and `a{oa{sv}}` are also
/// supported: fields whose value type is itself a dict (or any non-`Variant` type) are
/// serialized directly through their own `Serialize` impl rather than wrapped as a variant.
///
/// # Alternative Approaches
///
/// There are two approaches to serializing structs as dictionaries:
///
/// 1. Using this macro (simpler, but less control).
/// 2. Using the `Serialize` derive with `zgvariant::as_value` (more verbose, but more control).
///
/// See the example below and the relevant [FAQ entry] for more details on the alternative
/// approach.
///
/// # Example
///
/// ## Approach #1
///
/// ```
/// use zgvariant::{SerializeDict, Type};
///
/// #[derive(Debug, Default, SerializeDict, Type)]
/// #[zgvariant(signature = "a{sv}", rename_all = "PascalCase")]
/// pub struct MyStruct {
///     field1: Option<u32>,
///     field2: String,
/// }
/// ```
///
/// ## Approach #2
///
/// ```
/// use serde::Serialize;
/// use zgvariant::{Type, as_value};
///
/// #[derive(Debug, Default, Serialize, Type)]
/// #[zgvariant(signature = "a{sv}")]
/// #[serde(default, rename_all = "PascalCase")]
/// pub struct MyStruct {
///     #[serde(with = "as_value::optional", skip_serializing_if = "Option::is_none")]
///     field1: Option<u32>,
///     #[serde(with = "as_value")]
///     field2: String,
/// }
/// ```
///
/// ## Nested dictionaries
///
/// To represent shapes like `a{sa{sv}}`, nest one `SerializeDict`/`DeserializeDict` struct
/// inside another:
///
/// ```
/// use zgvariant::{DeserializeDict, SerializeDict, Type};
///
/// #[derive(SerializeDict, DeserializeDict, Type, Default)]
/// #[zgvariant(signature = "a{sv}", rename_all = "PascalCase")]
/// pub struct AdapterProperties {
///     address: Option<String>,
///     name: Option<String>,
/// }
///
/// #[derive(SerializeDict, DeserializeDict, Type, Default)]
/// #[zgvariant(signature = "a{sa{sv}}")]
/// pub struct InterfaceProperties {
///     #[zgvariant(rename = "org.example.Adapter1")]
///     adapter: Option<AdapterProperties>,
/// }
/// ```
///
/// # Renaming fields
///
/// A `#[zgvariant(rename_all = "case")]` attribute on the struct renames all its fields from
/// snake case to another case; supported values are `"lowercase"`, `"UPPERCASE"`,
/// `"PascalCase"`, `"camelCase"`, `"snake_case"` and `"kebab-case"`. Individual fields can
/// still be renamed with `#[zgvariant(rename = "...")]`, which takes precedence over
/// `rename_all`.
///
/// # Custom crate path
///
/// If you've renamed `zgvariant` in your `Cargo.toml` or are using it through a re-export, you
/// can specify the crate path with `#[zgvariant(crate = "...")]`.
///
/// # Migrating from `zvariant`
///
/// The `#[zvariant(...)]` helper attribute is also accepted, as an alias for
/// `#[zgvariant(...)]`, so types deriving `zvariant::SerializeDict` today keep compiling
/// unchanged once switched over to deriving `zgvariant::SerializeDict`.
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [FAQ entry]: https://z-galaxy.github.io/zbus/faq.html#how-to-use-a-struct-as-a-dictionary
#[proc_macro_derive(SerializeDict, attributes(zgvariant, zvariant))]
pub fn serialize_dict_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    derive::expand_serialize_dict_derive(input, &config())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Adds a [`Deserialize`] implementation to structs to be deserialized from a dictionary type.
///
/// The dictionary type is determined by the `signature` attribute. The default is `a{sv}`
/// (string keys, variant values), but nested forms like `a{sa{sv}}` and `a{oa{sv}}` are also
/// supported: fields whose value type is itself a dict (or any non-`Variant` type) are
/// deserialized directly through their own `Deserialize` impl rather than unwrapped from a
/// variant. See [`SerializeDict`] for a nested example, and for the field-renaming and
/// custom-crate-path attributes, which apply here too.
///
/// # Alternative Approaches
///
/// There are two approaches to deserializing dictionaries as structs:
///
/// 1. Using this macro (simpler, but less control).
/// 2. Using the `Deserialize` derive with `zgvariant::as_value` (more verbose, but more control).
///
/// See the example below and the relevant [FAQ entry] for more details on the alternative
/// approach.
///
/// # Example
///
/// ## Approach #1
///
/// ```
/// use zgvariant::{DeserializeDict, Type};
///
/// #[derive(Debug, Default, DeserializeDict, Type)]
/// #[zgvariant(signature = "a{sv}", rename_all = "PascalCase")]
/// pub struct MyStruct {
///     field1: Option<u32>,
///     field2: String,
/// }
/// ```
///
/// ## Approach #2
///
/// ```
/// use serde::Deserialize;
/// use zgvariant::{Type, as_value};
///
/// #[derive(Debug, Default, Deserialize, Type)]
/// #[zgvariant(signature = "a{sv}")]
/// #[serde(default, rename_all = "PascalCase")]
/// pub struct MyStruct {
///     #[serde(with = "as_value::optional")]
///     field1: Option<u32>,
///     #[serde(with = "as_value")]
///     field2: String,
/// }
/// ```
///
/// # Migrating from `zvariant`
///
/// The `#[zvariant(...)]` helper attribute is also accepted, as an alias for
/// `#[zgvariant(...)]`, so types deriving `zvariant::DeserializeDict` today keep compiling
/// unchanged once switched over to deriving `zgvariant::DeserializeDict`.
///
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [`SerializeDict`]: crate::SerializeDict
/// [FAQ entry]: https://z-galaxy.github.io/zbus/faq.html#how-to-use-a-struct-as-a-dictionary
#[proc_macro_derive(DeserializeDict, attributes(zgvariant, zvariant))]
pub fn deserialize_dict_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    derive::expand_deserialize_dict_derive(input, &config())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Implements conversions for your type to/from [`Value`].
///
/// Implements `From<Self>` for `Value` and `TryFrom<Value>` back to `Self`, for both structs
/// (any number of fields, including generic ones) and enums (unit variants only, encoded by
/// default as `u32`, or as a string when combined with `#[zgvariant(signature = "s")]`).
///
/// # Examples
///
/// Simple owned structures:
///
/// ```
/// use zgvariant::{OwnedObjectPath, OwnedValue, Value};
///
/// #[derive(Clone, Value, OwnedValue)]
/// struct OwnedStruct {
///     owned_str: String,
///     owned_path: OwnedObjectPath,
/// }
///
/// let s = OwnedStruct {
///     owned_str: String::from("hi"),
///     owned_path: OwnedObjectPath::try_from("/blah").unwrap(),
/// };
/// let value = Value::from(s.clone());
/// let _ = OwnedStruct::try_from(value).unwrap();
/// let value = OwnedValue::try_from(s).unwrap();
/// let s = OwnedStruct::try_from(value).unwrap();
/// assert_eq!(s.owned_str, "hi");
/// assert_eq!(s.owned_path.as_str(), "/blah");
/// ```
///
/// Now for the more exciting case of unowned structures:
///
/// ```
/// use zgvariant::{ObjectPath, Str};
/// # use zgvariant::{OwnedValue, Value};
/// #
/// #[derive(Clone, Value, OwnedValue)]
/// struct UnownedStruct<'a> {
///     s: Str<'a>,
///     path: ObjectPath<'a>,
/// }
///
/// let hi = String::from("hi");
/// let s = UnownedStruct {
///     s: Str::from(&hi),
///     path: ObjectPath::try_from("/blah").unwrap(),
/// };
/// let value = Value::from(s.clone());
/// let s = UnownedStruct::try_from(value).unwrap();
///
/// let value = OwnedValue::try_from(s).unwrap();
/// let s = UnownedStruct::try_from(value).unwrap();
/// assert_eq!(s.s, "hi");
/// assert_eq!(s.path, "/blah");
/// ```
///
/// Generic structures also supported:
///
/// ```
/// # use zgvariant::{OwnedObjectPath, OwnedValue, Value};
/// #
/// #[derive(Clone, Value, OwnedValue)]
/// struct GenericStruct<S, O> {
///     field1: S,
///     field2: O,
/// }
///
/// let s = GenericStruct {
///     field1: String::from("hi"),
///     field2: OwnedObjectPath::try_from("/blah").unwrap(),
/// };
/// let value = Value::from(s.clone());
/// let _ = GenericStruct::<String, OwnedObjectPath>::try_from(value).unwrap();
/// let value = OwnedValue::try_from(s).unwrap();
/// let s = GenericStruct::<String, OwnedObjectPath>::try_from(value).unwrap();
/// assert_eq!(s.field1, "hi");
/// assert_eq!(s.field2.as_str(), "/blah");
/// ```
///
/// Enums also supported but currently only with unit variants:
///
/// ```
/// # use zgvariant::{OwnedValue, Value};
/// #
/// #[derive(Debug, PartialEq, Value, OwnedValue)]
/// // Default representation is `u32`.
/// #[repr(u8)]
/// enum Enum {
///     Variant1 = 0,
///     Variant2,
/// }
///
/// let value = Value::from(Enum::Variant1);
/// let e = Enum::try_from(value).unwrap();
/// assert_eq!(e, Enum::Variant1);
/// assert_eq!(e as u8, 0);
/// let value = OwnedValue::try_from(Enum::Variant2).unwrap();
/// let e = Enum::try_from(value).unwrap();
/// assert_eq!(e, Enum::Variant2);
/// ```
///
/// String-encoded enums are also supported:
///
/// ```
/// # use zgvariant::{OwnedValue, Value};
/// #
/// #[derive(Debug, PartialEq, Value, OwnedValue)]
/// #[zgvariant(signature = "s")]
/// enum StrEnum {
///     Variant1,
///     Variant2,
/// }
///
/// let value = Value::from(StrEnum::Variant1);
/// let e = StrEnum::try_from(value).unwrap();
/// assert_eq!(e, StrEnum::Variant1);
/// let value = OwnedValue::try_from(StrEnum::Variant2).unwrap();
/// let e = StrEnum::try_from(value).unwrap();
/// assert_eq!(e, StrEnum::Variant2);
/// ```
///
/// # Renaming fields
///
/// ## Auto Renaming
///
/// The macro supports specifying a serde-like `#[zgvariant(rename_all = "case")]` attribute on
/// structures. The attribute allows to rename all the fields from snake case to another case
/// automatically.
///
/// Currently the macro supports the following values for `case`:
///
/// * `"lowercase"`
/// * `"UPPERCASE"`
/// * `"PascalCase"`
/// * `"camelCase"`
/// * `"snake_case"`
/// * `"kebab-case"`
///
/// ## Individual Fields
///
/// It's still possible to specify custom names for individual fields using the
/// `#[zgvariant(rename = "another-name")]` attribute even when the `rename_all` attribute is
/// present.
///
/// Here is an example using both `rename` and `rename_all`:
///
/// ```
/// # use zgvariant::{OwnedValue, Value, Dict};
/// # use std::collections::HashMap;
/// #
/// #[derive(Clone, Value, OwnedValue)]
/// #[zgvariant(signature = "dict", rename_all = "PascalCase")]
/// struct RenamedStruct {
///     #[zgvariant(rename = "MyValue")]
///     field1: String,
///     field2: String,
/// }
///
/// let s = RenamedStruct {
///     field1: String::from("hello"),
///     field2: String::from("world")
/// };
/// let v = Value::from(s);
/// let d = Dict::try_from(v).unwrap();
/// let hm: HashMap<String, String> = HashMap::try_from(d).unwrap();
/// assert_eq!(hm.get("MyValue").unwrap().as_str(), "hello");
/// assert_eq!(hm.get("Field2").unwrap().as_str(), "world");
/// ```
///
/// # Dictionary encoding
///
/// To treat your type as a dictionary, use `#[zgvariant(signature = "dict")]` (or an explicit
/// `a{sv}`/`a{s?}` signature). See [`Type`] for more details.
///
/// # Custom crate path
///
/// If you've renamed `zgvariant` in your `Cargo.toml` or are using it through a re-export, you
/// can specify the crate path with `#[zgvariant(crate = "...")]`:
///
/// ```
/// use zgvariant::Value;
///
/// #[derive(Clone, Value)]
/// #[zgvariant(crate = "zgvariant")]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// # Migrating from `zvariant`
///
/// The `#[zvariant(...)]` helper attribute is also accepted, as an alias for
/// `#[zgvariant(...)]`, so types deriving `zvariant::Value` today keep compiling unchanged once
/// switched over to deriving `zgvariant::Value`.
///
/// [`Value`]: https://docs.rs/zgvariant/latest/zgvariant/enum.Value.html
/// [`Type`]: crate::Type#custom-signatures
#[proc_macro_derive(Value, attributes(zgvariant, zvariant))]
pub fn value_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    derive::expand_value_derive(ast, derive::ValueType::Value, &config())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Implements conversions for your type to/from [`OwnedValue`].
///
/// Implements `TryFrom<Self>` for `OwnedValue` and `TryFrom<OwnedValue>` back to `Self`.
///
/// See [`Value`] documentation for the supported attributes and their semantics; the same
/// `#[zgvariant(...)]`/`#[zvariant(...)]` attributes apply here.
///
/// [`OwnedValue`]: https://docs.rs/zgvariant/latest/zgvariant/struct.OwnedValue.html
/// [`Value`]: crate::Value
#[proc_macro_derive(OwnedValue, attributes(zgvariant, zvariant))]
pub fn owned_value_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    derive::expand_value_derive(ast, derive::ValueType::OwnedValue, &config())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Constructs a const [`Signature`] with compile-time validation.
///
/// This macro builds a `Signature` from a string literal at compile time, validating that the
/// string is a valid GVariant signature; invalid signatures are rejected with a compilation
/// error. The `dict` alias is accepted as shorthand for `a{sv}`, just like in the `signature`
/// derive attribute.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use zgvariant::signature;
///
/// // Create signatures for basic types
/// let sig = signature!("s"); // String signature
/// assert_eq!(sig.to_string(), "s");
///
/// let sig = signature!("i"); // 32-bit integer signature
/// assert_eq!(sig.to_string(), "i");
/// ```
///
/// ## Container types
///
/// ```
/// use zgvariant::signature;
///
/// // Array of strings
/// let sig = signature!("as");
/// assert_eq!(sig.to_string(), "as");
///
/// // Dictionary mapping strings to variants
/// let sig = signature!("a{sv}");
/// assert_eq!(sig.to_string(), "a{sv}");
///
/// // Structures
/// let sig = signature!("(isx)");
/// assert_eq!(sig.to_string(), "(isx)");
/// ```
///
/// ## Const signatures
///
/// The macro can be used to create const signatures, which is especially useful for defining
/// signatures at compile time:
///
/// ```
/// use zgvariant::{signature, Signature};
///
/// const MY_SIGNATURE: Signature = signature!("a{sv}");
///
/// fn process_data(_data: &str) {
///     assert_eq!(MY_SIGNATURE.to_string(), "a{sv}");
/// }
/// ```
///
/// ## Using the `dict` alias
///
/// For convenience, `dict` is an alias for `a{sv}` (string-to-variant dictionary):
///
/// ```
/// use zgvariant::signature;
///
/// let sig = signature!("dict");
/// assert_eq!(sig.to_string(), "a{sv}");
/// ```
///
/// ## Compile-time validation
///
/// Invalid signatures will be caught at compile time:
///
/// ```compile_fail
/// use zgvariant::signature;
///
/// // This will fail to compile because 'z' is not a valid GVariant type
/// let sig = signature!("z");
/// ```
///
/// [`Signature`]: https://docs.rs/zgvariant/latest/zgvariant/enum.Signature.html
#[proc_macro]
pub fn signature(input: TokenStream) -> TokenStream {
    derive::expand_signature_macro(input.into(), &quote! { ::zgvariant })
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;
    use zvariant_utils::derive;

    #[test]
    fn type_derive_emits_zgvariant_paths() {
        let ast: syn::DeriveInput = parse_quote! {
            struct Foo { bar: u32 }
        };
        let tokens = derive::expand_type_derive(ast, &super::config())
            .unwrap()
            .to_string();
        assert!(tokens.replace(' ', "").contains("::zgvariant::Type"));
    }

    #[test]
    fn crate_attribute_overrides_default_path() {
        let ast: syn::DeriveInput = parse_quote! {
            #[zgvariant(crate = "my_crate")]
            struct Foo { bar: u32 }
        };
        let tokens = derive::expand_type_derive(ast, &super::config())
            .unwrap()
            .to_string();
        assert!(tokens.contains("my_crate"));
    }

    #[test]
    fn conflicting_namespaces_error() {
        let ast: syn::DeriveInput = parse_quote! {
            #[zgvariant(signature = "s")]
            #[zvariant(signature = "s")]
            struct Foo(String);
        };
        let err = derive::expand_type_derive(ast, &super::config()).unwrap_err();
        assert!(err.to_string().contains("duplicate"));
    }
}
