use super::{super::normal::*, iterator::*};

use {problemo::*, std::collections::*};

//
// KeyValuePairIteratorForHashMap
//

/// A [KeyValuePairIterator] for [HashMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForHashMap<'this, AnnotatedT> {
    /// Inner iterator.
    pub inner: hash_map::Iter<'this, Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<'this, AnnotatedT> KeyValuePairIteratorForHashMap<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: hash_map::Iter<'this, Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'this HashMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'this, AnnotatedT> KeyValuePairIterator<AnnotatedT> for KeyValuePairIteratorForHashMap<'this, AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<Option<(&'this Variant<AnnotatedT>, &'this Variant<AnnotatedT>)>, (Problem, &Variant<AnnotatedT>)> {
        Ok(self.inner.next())
    }
}

//
// IntoKeyValuePairIteratorForHashMap
//

/// An [IntoKeyValuePairIterator] for [HashMap].
///
/// It's just a simple wrapper.
pub struct IntoKeyValuePairIteratorForHashMap<AnnotatedT> {
    /// Inner iterator.
    pub inner: hash_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<AnnotatedT> IntoKeyValuePairIteratorForHashMap<AnnotatedT> {
    /// Constructor.
    pub fn new(inner: hash_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: HashMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<AnnotatedT> IntoKeyValuePairIterator<AnnotatedT> for IntoKeyValuePairIteratorForHashMap<AnnotatedT> {
    fn next(&mut self) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (Problem, Variant<AnnotatedT>)> {
        Ok(self.inner.next())
    }
}
