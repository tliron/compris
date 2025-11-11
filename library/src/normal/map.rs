use super::{
    super::{annotate::*, depict::*, kv::*},
    list::*,
    macros::*,
    variant::*,
};

use {
    depiction::*,
    kutil::std::iter::*,
    problemo::*,
    std::{
        collections::*,
        fmt::{self, Write},
        io,
        mem::*,
    },
};

//
// Map
//

impl_normal! {
    /// Normal map variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    Map(BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>)
}

impl<AnnotatedT> Map<AnnotatedT> {
    /// Get.
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Variant<AnnotatedT>>
    where
        KeyT: Into<Variant<AnnotatedT>>,
    {
        self.inner.get(&key.into())
    }

    /// Get.
    pub fn into_get_mut<KeyT>(&mut self, key: KeyT) -> Option<&mut Variant<AnnotatedT>>
    where
        KeyT: Into<Variant<AnnotatedT>>,
    {
        self.inner.get_mut(&key.into())
    }

    /// Insert.
    pub fn into_insert<KeyT, ValueT>(&mut self, key: KeyT, value: ValueT) -> Option<Variant<AnnotatedT>>
    where
        KeyT: Into<Variant<AnnotatedT>>,
        ValueT: Into<Variant<AnnotatedT>>,
    {
        self.inner.insert(key.into(), value.into())
    }

    /// Remove.
    pub fn into_remove<KeyT>(&mut self, key: KeyT) -> Option<Variant<AnnotatedT>>
    where
        KeyT: Into<Variant<AnnotatedT>>,
    {
        self.inner.remove(&key.into())
    }

    /// True if any of the map keys is a collection.
    pub fn has_a_collection_key(&self) -> bool {
        for key in self.inner.keys() {
            if key.is_collection() {
                return true;
            }
        }
        false
    }

    /// If the map has *only* one key then returns the key-value tuple.
    pub fn to_key_value_pair(&self) -> Option<(&Variant<AnnotatedT>, &Variant<AnnotatedT>)> {
        match self.inner.len() {
            1 => return self.inner.iter().next(),
            _ => None,
        }
    }

    /// If the map has *only* one key then returns the key-value tuple.
    pub fn into_key_value_pair(self) -> Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)> {
        match self.inner.len() {
            1 => return self.inner.into_iter().next(),
            _ => None,
        }
    }

    /// Removes all entries from the map and returns them as a vector of key-value tuples.
    pub fn into_vector(&mut self) -> Vec<(Variant<AnnotatedT>, Variant<AnnotatedT>)> {
        take(&mut self.inner).into_iter().collect()
    }

    /// [Depict] with [Annotations].
    pub fn annotated_depiction(&self) -> AnnotatedMapDepiction<'_, AnnotatedT> {
        AnnotatedMapDepiction::new(self)
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<Map<NewAnnotatedT>> for Map<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(mut self) -> Map<NewAnnotatedT> {
        let new_map: Map<NewAnnotatedT> =
            self.into_vector().into_iter().map(|(key, value)| (key.into_annotated(), value.into_annotated())).collect();
        if AnnotatedT::can_have_annotations()
            && NewAnnotatedT::can_have_annotations()
            && let Some(annotations) = self.annotated.annotations()
        {
            new_map.with_annotations(annotations.clone())
        } else {
            new_map
        }
    }
}

impl<AnnotatedT> RemoveAnnotations<Map<WithoutAnnotations>> for Map<AnnotatedT> {
    fn remove_annotations(self) -> Map<WithoutAnnotations> {
        self.inner.into_iter().map(|(key, value)| (key.remove_annotations(), value.remove_annotations())).collect()
    }
}

impl<AnnotatedT> Depict for Map<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        // Upgrade reduced to verbose if there are collection keys
        let override_format = if (context.get_format() == DepictionFormat::Optimized) && self.has_a_collection_key() {
            Some(DepictionFormat::Verbose)
        } else {
            None
        };

        depict_map(&self.inner, override_format, writer, context)
    }
}

impl<AnnotatedT> fmt::Display for Map<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_char('{')?;

        for ((key, value), last) in IterateWithLast::new(self) {
            fmt::Display::fmt(key, formatter)?;
            formatter.write_char(':')?;
            fmt::Display::fmt(value, formatter)?;
            if !last {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char('}')
    }
}

// Iterators

impl<AnnotatedT> IntoIterator for Map<AnnotatedT> {
    type Item = (Variant<AnnotatedT>, Variant<AnnotatedT>);
    type IntoIter = btree_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'this, AnnotatedT> IntoIterator for &'this Map<AnnotatedT> {
    type Item = (&'this Variant<AnnotatedT>, &'this Variant<AnnotatedT>);
    type IntoIter = btree_map::Iter<'this, Variant<AnnotatedT>, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'this, AnnotatedT> IntoIterator for &'this mut Map<AnnotatedT> {
    type Item = (&'this Variant<AnnotatedT>, &'this mut Variant<AnnotatedT>);
    type IntoIter = btree_map::IterMut<'this, Variant<AnnotatedT>, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

// Conversions

impl<const SIZE: usize, AnnotatedT> From<[(Variant<AnnotatedT>, Variant<AnnotatedT>); SIZE]> for Map<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(array: [(Variant<AnnotatedT>, Variant<AnnotatedT>); SIZE]) -> Self {
        BTreeMap::from(array).into()
    }
}

impl<AnnotatedT> FromIterator<(Variant<AnnotatedT>, Variant<AnnotatedT>)> for Map<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Variant<AnnotatedT>, Variant<AnnotatedT>)>,
    {
        BTreeMap::from_iter(iterator).into()
    }
}

impl<AnnotatedT> TryFrom<List<AnnotatedT>> for Map<AnnotatedT>
where
    AnnotatedT: Clone + Default,
{
    type Error = Problem;

    /// The iterated values are expected to be [List] of length 2 (key-value pairs).
    ///
    /// Will return an error if it encounters a duplicate key.
    fn try_from(list: List<AnnotatedT>) -> Result<Self, Self::Error> {
        let mut map = Self::default();

        let mut iterator = KeyValuePairIteratorForVariantIterator::new_for(&list);
        while let Some((key, value)) = iterator.next().map_err(|(error, _value)| error)? {
            map.inner.insert(key.clone(), value.clone());
        }

        Ok(map)
    }
}
