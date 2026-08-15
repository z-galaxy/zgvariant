use zgvariant::BE;

#[test]
fn i64_value() {
    basic_type_test!(BE, -0x0ABB_AABB_AABB_AAB0_i64, 8, i64, 8, I64, 10);
}
