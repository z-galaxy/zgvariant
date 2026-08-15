use zgvariant::BE;

#[test]
fn i16_value() {
    basic_type_test!(BE, -0xAB0_i16, 2, i16, 2, I16, 4);
}
