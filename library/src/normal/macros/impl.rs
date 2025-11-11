/// Helper macro for implementing normal types.
#[macro_export]
macro_rules! impl_normal {
    ( $( #[$meta:meta] )* $type:ident ( $inner:ty ) $(,)? ) => {
        $( #[$meta] )*
        #[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::default::Default)]
        pub struct $type<AnnotatedT> {
            /// Inner.
            pub inner: $inner,

            /// Annotated.
            pub annotated: AnnotatedT,
        }

        $crate::impl_annotated!($type);

        impl<AnnotatedT> ::std::cmp::PartialEq for $type<AnnotatedT> {
            fn eq(&self, other: &Self) -> bool {
                self.inner.eq(&other.inner)
            }
        }

        impl<AnnotatedT> ::std::cmp::Eq for $type<AnnotatedT> {}

        impl<AnnotatedT> ::std::cmp::PartialOrd for $type<AnnotatedT> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                self.inner.partial_cmp(&other.inner)
            }
        }

        impl<AnnotatedT> ::std::cmp::Ord for $type<AnnotatedT> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }

        impl<AnnotatedT> ::std::hash::Hash for $type<AnnotatedT> {
            fn hash<HasherT>(&self, state: &mut HasherT)
            where
                HasherT: ::std::hash::Hasher,
            {
                self.inner.hash(state);
            }
        }

        impl<AnnotatedT> From<$inner> for $type<AnnotatedT>
        where
            AnnotatedT: ::std::default::Default,
        {
            fn from(inner: $inner) -> Self {
                Self { inner, annotated: Default::default() }
            }
        }

        impl<AnnotatedT> Into<$inner> for $type<AnnotatedT> {
            fn into(self) -> $inner {
                self.inner
            }
        }

        impl<'this, AnnotatedT> Into<&'this $inner> for &'this $type<AnnotatedT> {
            fn into(self) -> &'this $inner {
                &self.inner
            }
        }
    }
}

/// Helper macro for implementing normal types.
#[macro_export]
macro_rules! impl_normal_basic {
    ( $type:ident $(,)? ) => {
        impl<AnnotatedT> RemoveAnnotations<$type<WithoutAnnotations>> for $type<AnnotatedT> {
            fn remove_annotations(self) -> $type<WithoutAnnotations> {
                $type::from(self.inner)
            }
        }

        impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<$type<NewAnnotatedT>> for $type<AnnotatedT>
        where
            AnnotatedT: Annotated,
            NewAnnotatedT: Annotated + Default,
        {
            fn into_annotated(self) -> $type<NewAnnotatedT> {
                let new_self = $type::from(self.inner);
                if AnnotatedT::can_have_annotations()
                    && NewAnnotatedT::can_have_annotations()
                    && let Some(annotations) = self.annotated.annotations()
                {
                    new_self.with_annotations(annotations.clone())
                } else {
                    new_self
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub use {impl_normal, impl_normal_basic};
