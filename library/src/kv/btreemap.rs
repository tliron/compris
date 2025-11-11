use super::{
    super::{errors::*, normal::*},
    iterator::*,
};

use std::collections::*;

//
// KeyValuePairIteratorForBTreeMap
//

/// A [KeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForBTreeMap<'this, AnnotatedT> {
    /// Inner.
    pub inner: btree_map::Iter<'this, Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<'this, AnnotatedT> KeyValuePairIteratorForBTreeMap<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: btree_map::Iter<'this, Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'this BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'this, AnnotatedT> KeyValuePairIterator<AnnotatedT> for KeyValuePairIteratorForBTreeMap<'this, AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<Option<(&'this Variant<AnnotatedT>, &'this Variant<AnnotatedT>)>, (MalformedError, &Variant<AnnotatedT>)>
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
    fn next(
        &mut self,
    ) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (MalformedError, Variant<AnnotatedT>)> {
        Ok(self.inner.next())
    }
}
