// Test through both generic and specific API (wrt byte order)
#[macro_export]
macro_rules! basic_type_test {
    ($endian:expr, $test_value:expr, $expected_len:expr, $expected_ty:ty, $align:literal) => {{
        // Lie that we're starting at byte 1 in the overall message to test padding
        let ctxt = zgvariant::serialized::Context::new($endian, 1);
        let encoded = zgvariant::to_bytes(ctxt, &$test_value).unwrap();
        let padding = zgvariant::padding_for_n_bytes(1, $align);

        assert_eq!(
            encoded.len(),
            $expected_len + padding,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): ($expected_ty, _) = encoded.deserialize().unwrap();
        assert!(decoded == $test_value, "invalid decoding");
        assert!(parsed == encoded.len(), "invalid parsing");

        // Now encode w/o padding
        let ctxt = zgvariant::serialized::Context::new($endian, 0);
        let encoded = zgvariant::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );

        encoded
    }};
    ($endian:expr, $test_value:expr, $expected_len:expr, $expected_ty:ty, $align:literal, $kind:ident, $expected_value_len:expr) => {{
        let encoded = basic_type_test!($endian, $test_value, $expected_len, $expected_ty, $align);

        // As Value
        let v: zgvariant::Value<'_> = $test_value.into();
        assert_eq!(
            v.value_signature(),
            <$expected_ty as zgvariant::Basic>::SIGNATURE_STR
        );
        assert_eq!(v, zgvariant::Value::$kind($test_value));
        value_test!($endian, v, $expected_value_len);

        let v: $expected_ty = v.try_into().unwrap();
        assert_eq!(v, $test_value);

        encoded
    }};
}

#[macro_export]
macro_rules! value_test {
    ($endian:expr, $test_value:expr, $expected_len:expr) => {{
        let ctxt = zgvariant::serialized::Context::new($endian, 0);
        let encoded = zgvariant::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): (zgvariant::Value<'_>, _) = encoded.deserialize().unwrap();
        assert!(decoded == $test_value, "invalid decoding");
        assert!(parsed == encoded.len(), "invalid parsing");

        encoded
    }};
}
