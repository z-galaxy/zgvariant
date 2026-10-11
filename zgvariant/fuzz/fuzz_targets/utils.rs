use zgvariant::{
    serialized::{Context, Data},
    to_bytes, Variant,
};

pub fn fuzz_for_context(bytes: &[u8], ctx: Context) {
    let data = Data::new(bytes, ctx);
    if let Ok((decoded, _)) = data.deserialize::<Variant>() {
        to_bytes(ctx, &decoded).unwrap();
    }
}
