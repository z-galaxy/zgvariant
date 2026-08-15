use zgvariant::BE;

#[test]
fn i32_value() {
    basic_type_test!(BE, -0x0ABB_AAB0_i32, 4, i32, 4, I32, 6);
}
