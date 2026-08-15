#[cfg(unix)]
use zgvariant::{Error, LE, Signature, serialized::Context, to_bytes_for_signature};

// The `h` (file descriptor) signature is not supported by the GVariant format (there's no
// out-of-band fd-passing channel to route it through), so ser/de must reject it cleanly
// instead of panicking.
#[test]
#[cfg(unix)]
fn fd_signature_is_rejected() {
    let ctxt = Context::new(LE, 0);
    let sig = Signature::Fd;
    let err = to_bytes_for_signature(ctxt, &sig, &0i32).unwrap_err();
    assert!(matches!(err, Error::UnsupportedType(_)));

    let data = to_bytes_for_signature(ctxt, "u", &0u32).unwrap();
    let err = data.deserialize_for_signature::<_, i32>(&sig).unwrap_err();
    assert!(matches!(err, Error::UnsupportedType(_)));
}
