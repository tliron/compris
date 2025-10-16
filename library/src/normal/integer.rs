use super::{super::annotate::*, macros::*};

use {
    depiction::*,
    duplicate::*,
    std::{fmt, io},
};

//
// Integer
//

impl_normal! {
    /// Normal integer variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    Integer(i64)
}

impl_normal_basic!(Integer);

impl<AnnotatedT> Depict for Integer<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        if context.get_format() == DepictionFormat::Compact {
            write!(writer, "{}", context.theme.number(format!("{:+}", self.inner)))
        } else {
            write!(writer, "{} {}", context.theme.number(self.inner), context.theme.meta("i64"))
        }
    }
}

impl<AnnotatedT> fmt::Display for Integer<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}i64", self.inner)
    }
}

// Conversions

#[duplicate_item(
  FromT;
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl<AnnotatedT> From<FromT> for Integer<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(integer: FromT) -> Self {
        Self::from(integer as i64)
    }
}

impl<AnnotatedT> From<&Integer<AnnotatedT>> for i64 {
    fn from(integer: &Integer<AnnotatedT>) -> Self {
        integer.inner
    }
}
