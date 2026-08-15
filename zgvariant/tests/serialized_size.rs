use zgvariant::{LE, serialized::Context, serialized_size};

#[test]
fn test_serialized_size() {
    let ctxt = Context::new(LE, 0);
    let l = serialized_size(ctxt, &()).unwrap();
    assert_eq!(*l, 1);

    let l = serialized_size(ctxt, &('a', "abc", &(1_u32, 2))).unwrap();
    assert_eq!(*l, 18);

    let v = vec![1, 2];
    let l = serialized_size(ctxt, &('a', "abc", &v)).unwrap();
    assert_eq!(*l, 18);
}
