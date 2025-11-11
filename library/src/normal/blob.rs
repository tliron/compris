use super::{super::annotate::*, macros::*};

use {
    depiction::*,
    duplicate::*,
    kutil::std::immutable::*,
    std::{borrow::*, fmt, io},
};

//
// Blob
//

impl_normal! {
    /// Normal blob variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is a [Bytes] in order to support zero-copy cloning.
    Blob(Bytes)
}

impl_normal_basic!(Blob);

impl<AnnotatedT> Blob<AnnotatedT> {
    /// Constructor.
    pub fn new_from_base64<BytesT>(base64: BytesT) -> Result<Self, base64_simd::Error>
    where
        AnnotatedT: Default,
        BytesT: AsRef<[u8]>,
    {
        let bytes = base64_simd::STANDARD.decode_to_vec(base64)?;
        Ok(Self::from(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        base64_simd::STANDARD.encode_to_string(&self.inner)
    }

    /// As slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }
}

impl<AnnotatedT> Depict for Blob<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, format!("{} bytes", self.inner.len()))
    }
}

impl<AnnotatedT> fmt::Display for Blob<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} bytes", self.inner.len())
    }
}

// Conversion

impl<AnnotatedT> AsRef<[u8]> for Blob<AnnotatedT> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<AnnotatedT> From<&'static [u8]> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(bytes: &'static [u8]) -> Self {
        let bytes = Bytes::from_static(bytes);
        Blob::from(bytes)
    }
}

impl<AnnotatedT> From<Vec<u8>> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(bytes: Vec<u8>) -> Self {
        Blob::from(Bytes::from(bytes))
    }
}

impl<AnnotatedT> From<Cow<'_, [u8]>> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(bytes: Cow<'_, [u8]>) -> Self {
        match bytes {
            Cow::Borrowed(bytes) => bytes.to_vec().into(),
            Cow::Owned(bytes) => bytes.into(),
        }
    }
}

#[duplicate_item(
  FromT;
  [ByteString];
  [String];
  [&str];
)]
impl<AnnotatedT> From<FromT> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: FromT) -> Self {
        ByteString::from(string).into_bytes().into()
    }
}

impl<AnnotatedT> From<Cow<'_, str>> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: Cow<'_, str>) -> Self {
        match string {
            Cow::Borrowed(string) => string.into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl<AnnotatedT> From<Blob<AnnotatedT>> for Vec<u8> {
    fn from(blob: Blob<AnnotatedT>) -> Self {
        blob.inner.into()
    }
}

impl<'this, AnnotatedT> From<&'this Blob<AnnotatedT>> for &'this [u8] {
    fn from(blob: &'this Blob<AnnotatedT>) -> Self {
        &blob.inner
    }
}
