use super::{super::normal::*, iterator::*};

use {problemo::*, std::collections::*};

//
// KeyValuePairIteratorForBTreeMap
//

/// A [KeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForBTreeMap<'inner, AnnotatedT> {
    /// Inner.
    pub inner: btree_map::Iter<'inner, Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<'inner, AnnotatedT> KeyValuePairIteratorForBTreeMap<'inner, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: btree_map::Iter<'inner, Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'inner BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'inner, AnnotatedT> KeyValuePairIterator<AnnotatedT> for KeyValuePairIteratorForBTreeMap<'inner, AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<Option<(&'inner Variant<AnnotatedT>, &'inner Variant<AnnotatedT>)>, (Problem, &Variant<AnnotatedT>)>
    {
        Ok(self.inner.next())
    }
}

//
// KeyValuePairIteratorForBTreeMap
//

/// An [IntoKeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct IntoKeyValuePairIteratorForBTreeMap<AnnotatedT> {
    /// Inner.
    pub inner: btree_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<AnnotatedT> IntoKeyValuePairIteratorForBTreeMap<AnnotatedT> {
    /// Constructor.
    pub fn new(inner: btree_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<AnnotatedT> IntoKeyValuePairIterator<AnnotatedT> for IntoKeyValuePairIteratorForBTreeMap<AnnotatedT> {
    fn next(&mut self) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (Problem, Variant<AnnotatedT>)> {
        Ok(self.inner.next())
    }
}
