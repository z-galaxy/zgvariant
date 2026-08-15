use serde::{Deserialize, Serialize};
#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::Type;
#[cfg(not(unix))]
use crate::{Error, Result};

/// File name represented as a nul-terminated byte array.
///
/// While `zgvariant::Type` and `serde::{Serialize, Deserialize}`, are implemented for [`Path`]
/// and [`PathBuf`], unfortunately `serde` serializes them as UTF-8 strings and that limits the
/// number of possible characters to use on a file path. This is not the desired behavior since
/// file paths are not guaranteed to contain only UTF-8 characters.
///
/// To solve this problem, this type is provided which encodes the underlying file path as a
/// null-terminated byte array.
///
/// # Converting back to standard types
///
/// On unix, paths are byte strings just like this type, so the conversions to [`OsString`],
/// [`PathBuf`], [`&OsStr`](OsStr) and [`&Path`](Path) are infallible [`From`] implementations that
/// preserve the bytes exactly. On other platforms no such lossless mapping exists, so the same
/// conversions are offered as [`TryFrom`] instead and succeed only for valid UTF-8. Code that
/// needs to build for both can always use the [`TryFrom`] form.
///
/// # Examples:
///
/// ```
/// use zgvariant::FilePath;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("/hello/world\0");
/// let path_buf = PathBuf::from(path);
///
/// let p1 = FilePath::from(path);
/// let p2 = FilePath::from(path_buf);
/// let p3 = FilePath::from("/hello/world");
///
/// assert_eq!(p1, p2);
/// assert_eq!(p2, p3);
/// ```
#[derive(Type, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone, Ord, PartialOrd)]
#[zgvariant(signature = "ay")]
pub struct FilePath<'f>(Cow<'f, CStr>);

impl<'f> FilePath<'f> {
    pub fn new(cow: Cow<'f, CStr>) -> Self {
        Self(cow)
    }

    /// Returns a lossy UTF-8 representation of the file path.
    ///
    /// Invalid UTF-8 sequences are replaced with `U+FFFD REPLACEMENT CHARACTER`.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.0.to_string_lossy()
    }
}

impl From<CString> for FilePath<'_> {
    fn from(value: CString) -> Self {
        FilePath(Cow::Owned(value))
    }
}

impl<'f> From<&'f CString> for FilePath<'f> {
    fn from(value: &'f CString) -> Self {
        FilePath(Cow::Borrowed(value.as_c_str()))
    }
}

impl<'f> From<&'f OsStr> for FilePath<'f> {
    fn from(value: &'f OsStr) -> FilePath<'f> {
        FilePath(bytes_with_null(value.as_encoded_bytes()))
    }
}

impl<'f> From<&'f OsString> for FilePath<'f> {
    fn from(value: &'f OsString) -> FilePath<'f> {
        FilePath(bytes_with_null(value.as_encoded_bytes()))
    }
}

impl From<OsString> for FilePath<'_> {
    fn from(value: OsString) -> Self {
        FilePath(Cow::Owned(
            bytes_with_null(value.as_encoded_bytes()).into_owned(),
        ))
    }
}

impl<'f> From<&'f PathBuf> for FilePath<'f> {
    fn from(value: &'f PathBuf) -> FilePath<'f> {
        FilePath::from(value.as_os_str())
    }
}

impl From<PathBuf> for FilePath<'_> {
    fn from(value: PathBuf) -> FilePath<'static> {
        FilePath::from(OsString::from(value))
    }
}

impl<'f> From<&'f Path> for FilePath<'f> {
    fn from(value: &'f Path) -> Self {
        Self::from(value.as_os_str())
    }
}

impl<'f> From<&'f CStr> for FilePath<'f> {
    fn from(value: &'f CStr) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'f> From<&'f str> for FilePath<'f> {
    fn from(value: &'f str) -> Self {
        Self::from(OsStr::new(value))
    }
}

impl<'f> AsRef<FilePath<'f>> for FilePath<'f> {
    fn as_ref(&self) -> &FilePath<'f> {
        self
    }
}

#[cfg(unix)]
impl<'f> From<&'f FilePath<'f>> for &'f OsStr {
    fn from(value: &'f FilePath<'f>) -> Self {
        OsStr::from_bytes(value.0.to_bytes())
    }
}

#[cfg(unix)]
impl<'f> From<&'f FilePath<'f>> for &'f Path {
    fn from(value: &'f FilePath<'f>) -> Self {
        Path::new(<&OsStr>::from(value))
    }
}

#[cfg(unix)]
impl From<FilePath<'_>> for OsString {
    fn from(value: FilePath<'_>) -> Self {
        OsString::from_vec(value.0.into_owned().into_bytes())
    }
}

#[cfg(unix)]
impl From<FilePath<'_>> for PathBuf {
    fn from(value: FilePath<'_>) -> Self {
        OsString::from(value).into()
    }
}

#[cfg(not(unix))]
impl<'f> TryFrom<&'f FilePath<'f>> for &'f OsStr {
    type Error = Error;

    fn try_from(value: &'f FilePath<'f>) -> Result<Self> {
        value.0.to_str().map(OsStr::new).map_err(Error::Utf8)
    }
}

#[cfg(not(unix))]
impl<'f> TryFrom<&'f FilePath<'f>> for &'f Path {
    type Error = Error;

    fn try_from(value: &'f FilePath<'f>) -> Result<Self> {
        value.0.to_str().map(Path::new).map_err(Error::Utf8)
    }
}

#[cfg(not(unix))]
impl TryFrom<FilePath<'_>> for OsString {
    type Error = Error;

    fn try_from(value: FilePath<'_>) -> Result<Self> {
        value.0.to_str().map(OsString::from).map_err(Error::Utf8)
    }
}

#[cfg(not(unix))]
impl TryFrom<FilePath<'_>> for PathBuf {
    type Error = Error;

    fn try_from(value: FilePath<'_>) -> Result<Self> {
        value.0.to_str().map(PathBuf::from).map_err(Error::Utf8)
    }
}

/// Converts a byte slice into a null-terminated [CStr].
///
/// Returns a borrowed [CStr] if the slice already contains a null byte; otherwise, returns an
/// owned [CStr] with a null byte appended.
///
/// # Returns
///
/// A [Cow<'_, CStr>] containing a *guaranteed* null-terminated string.
fn bytes_with_null(bytes: &[u8]) -> Cow<'_, CStr> {
    if let Ok(cstr) = CStr::from_bytes_until_nul(bytes) {
        return Cow::Borrowed(cstr);
    }
    // unwrap is fine, as we handled the null termination case above.
    Cow::Owned(CString::new(bytes).unwrap())
}

#[cfg(test)]
mod file_path_test {
    use super::*;
    use crate::Signature;
    use std::path::{Path, PathBuf};

    #[test]
    fn from_test() {
        let path = Path::new("/hello/world");
        let path_buf = PathBuf::from(path);
        let osstr = OsStr::new("/hello/world");
        let os_string = OsString::from("/hello/world");
        let cstr = CStr::from_bytes_until_nul("/hello/world\0".as_bytes()).unwrap_or_default();
        let cstring = CString::new("/hello/world").unwrap_or_default();

        let p1 = FilePath::from(path);
        let p2 = FilePath::from(path_buf);
        let p3 = FilePath::from(osstr);
        let p4 = FilePath::from(os_string);
        let p5 = FilePath::from(cstr);
        let p6 = FilePath::from(cstring);
        let p7 = FilePath::from("/hello/world");

        assert_eq!(p1, p2);
        assert_eq!(p2, p3);
        assert_eq!(p3, p4);
        assert_eq!(p4, p5);
        assert_eq!(p5, p6);
        assert_eq!(p5, p7);
    }

    #[test]
    fn filepath_signature() {
        assert_eq!(
            &Signature::static_array(&Signature::U8),
            FilePath::SIGNATURE
        );
    }

    #[cfg(unix)]
    #[test]
    fn into_test() {
        let path = Path::new("/hello/world");
        let file_path = FilePath::from(path);

        assert_eq!(<&OsStr>::from(&file_path), path.as_os_str());
        assert_eq!(<&Path>::from(&file_path), path);
        assert_eq!(OsString::from(file_path.clone()), path.as_os_str());
        assert_eq!(PathBuf::from(file_path), path);
    }

    /// Arbitrary bytes are exactly what the unix conversions exist for, so nothing may be lost or
    /// replaced along the way.
    #[cfg(unix)]
    #[test]
    fn non_utf8_into_test() {
        let os_str = OsStr::from_bytes(b"/hello/\xff\xfe/world");
        let file_path = FilePath::from(os_str);

        assert_eq!(<&OsStr>::from(&file_path), os_str);
        assert_eq!(<&Path>::from(&file_path), Path::new(os_str));
        assert_eq!(OsString::from(file_path.clone()), os_str);
        assert_eq!(PathBuf::from(file_path), Path::new(os_str));
    }

    /// Off unix the same conversions are fallible and only UTF-8 gets through.
    #[cfg(not(unix))]
    #[test]
    fn try_into_test() {
        let path = Path::new("/hello/world");
        let file_path = FilePath::from(path);

        assert_eq!(<&OsStr>::try_from(&file_path).unwrap(), path.as_os_str());
        assert_eq!(<&Path>::try_from(&file_path).unwrap(), path);
        assert_eq!(
            OsString::try_from(file_path.clone()).unwrap(),
            path.as_os_str()
        );
        assert_eq!(PathBuf::try_from(file_path).unwrap(), path);

        let non_utf8 = FilePath::from(c"/\xff\xfe");
        assert!(matches!(<&Path>::try_from(&non_utf8), Err(Error::Utf8(_))));
        assert!(matches!(PathBuf::try_from(non_utf8), Err(Error::Utf8(_))));
    }

    /// Whatever comes in, exactly one nul comes out, at the end.
    #[test]
    fn nul_termination() {
        // (input, expected nul-terminated output)
        let cases: [(&[u8], &[u8]); 5] = [
            (b"", b"\0"),
            (b"\0", b"\0"),
            (b"\x01\x02\0", b"\x01\x02\0"),
            (b"\0\0", b"\0"),
            (b"\x01\0\x02\0", b"\x01\0"),
        ];

        for (input, expected) in cases {
            let expected = CStr::from_bytes_with_nul(expected).unwrap();
            assert_eq!(bytes_with_null(input), Cow::Borrowed(expected));
        }
    }
}
