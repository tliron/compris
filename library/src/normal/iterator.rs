use super::variant::*;

use std::{mem::*, slice, vec};

//
// VariantIterator
//

/// If the variant is a [List](super::list::List), iterates its items. Otherwise just iterates
/// itself once.
pub enum VariantIterator<'this, AnnotatedT> {
    /// Iterator.
    Iterator(slice::Iter<'this, Variant<AnnotatedT>>),

    /// Variant.
    Variant(Option<&'this Variant<AnnotatedT>>),
}

impl<'this, AnnotatedT> VariantIterator<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(variant: &'this Variant<AnnotatedT>) -> Self {
        match variant {
            Variant::List(list) => Self::Iterator(list.inner.iter()),
            _ => Self::Variant(Some(variant)),
        }
    }
}

impl<'this, AnnotatedT> Iterator for VariantIterator<'this, AnnotatedT> {
    type Item = &'this Variant<AnnotatedT>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Iterator(iter) => iter.next(),
            Self::Variant(variant) => take(variant),
        }
    }
}

//
// IntoVariantIterator
//

/// If the variant is a [List](super::list::List), iterates its items. Otherwise just iterates
/// itself once.
pub enum IntoVariantIterator<AnnotatedT> {
    /// Iterator.
    Iterator(vec::IntoIter<Variant<AnnotatedT>>),

    /// Variant.
    Variant(Option<Variant<AnnotatedT>>),
}

impl<AnnotatedT> IntoVariantIterator<AnnotatedT> {
    /// Constructor.
    pub fn new(variant: Variant<AnnotatedT>) -> Self {
        match variant {
            Variant::List(list) => Self::Iterator(list.inner.into_iter()),
            _ => Self::Variant(Some(variant)),
        }
    }
}

impl<AnnotatedT> Iterator for IntoVariantIterator<AnnotatedT> {
    type Item = Variant<AnnotatedT>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Iterator(iter) => iter.next(),
            Self::Variant(variant) => take(variant),
        }
    }
}
