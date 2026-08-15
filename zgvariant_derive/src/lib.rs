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
/// For structs, this works like serde's `Serialize`/`Deserialize` derives: the signature is
/// built up from the signatures of the fields, in declaration order. Enums are supported too,
/// as long as every variant has the same number and types of fields (if any). By default the
/// discriminant of a unit-only enum is encoded as `u32`; a `#[repr(..)]` attribute picks a
/// different integer type instead (which also requires the [`serde_repr`] crate for
/// (de)serialization).
///
/// # Custom signatures
///
/// A `#[zgvariant(signature = "...")]` attribute lets you hardcode the signature instead of
/// deriving it from the fields. A common use is encoding a struct as a dictionary (signature
/// `a{sv}`), for which `dict` is provided as a convenient alias. Another is encoding a
/// unit-only enum as a string (signature `s`) rather than as an integer.
///
/// # Custom crate path
///
/// If you've renamed `zgvariant` in your `Cargo.toml` or are using it through a re-export, you
/// can specify the crate path with `#[zgvariant(crate = "...")]`.
///
/// # Migrating from `zvariant`
///
/// The `#[zvariant(...)]` helper attribute is also accepted, as an alias for
/// `#[zgvariant(...)]`, so types deriving `zvariant::Type` today keep compiling unchanged once
/// switched over to deriving `zgvariant::Type`.
///
/// See the crate-level documentation for usage examples.
///
/// [`Type`]: https://docs.rs/zgvariant/latest/zgvariant/trait.Type.html
/// [`serde_repr`]: https://crates.io/crates/serde_repr
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
/// See the crate-level documentation for usage examples.
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
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
/// variant. See [`SerializeDict`] for the field-renaming and custom-crate-path attributes,
/// which apply here too.
///
/// See the crate-level documentation for usage examples.
///
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [`SerializeDict`]: crate::SerializeDict
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
/// # Renaming fields
///
/// A `#[zgvariant(rename_all = "case")]` attribute on the struct or enum renames all its
/// fields/variants from snake case to another case; supported values are `"lowercase"`,
/// `"UPPERCASE"`, `"PascalCase"`, `"camelCase"`, `"snake_case"` and `"kebab-case"`. Individual
/// fields can still be renamed with `#[zgvariant(rename = "...")]`, which takes precedence
/// over `rename_all`.
///
/// # Dictionary encoding
///
/// To treat your type as a dictionary, use `#[zgvariant(signature = "dict")]` (or an explicit
/// `a{sv}`/`a{s?}` signature). See [`Type`] for more details.
///
/// # Custom crate path
///
/// If you've renamed `zgvariant` in your `Cargo.toml` or are using it through a re-export, you
/// can specify the crate path with `#[zgvariant(crate = "...")]`.
///
/// # Migrating from `zvariant`
///
/// The `#[zvariant(...)]` helper attribute is also accepted, as an alias for
/// `#[zgvariant(...)]`, so types deriving `zvariant::Value` today keep compiling unchanged once
/// switched over to deriving `zgvariant::Value`.
///
/// See the crate-level documentation for usage examples.
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
/// string is a valid D-Bus/GVariant signature; invalid signatures are rejected with a
/// compilation error. The `dict` alias is accepted as shorthand for `a{sv}`, just like in the
/// `signature` derive attribute.
///
/// See the crate-level documentation for usage examples.
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
