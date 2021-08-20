use std::cmp;

use crate::Reductor;

macro_rules! impl_min_max {
    ($type:ident, $cmp:path) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy)]
        pub struct $type<T>(pub T);

        impl<T> Reductor<T> for $type<T>
        where
            T: cmp::Ord,
        {
            #[inline]
            fn new(v: T) -> Self {
                Self(v)
            }

            #[inline]
            fn reduce(acc: Self, elem: T) -> Self {
                Self($cmp(acc.0, elem))
            }
        }

        impl<T> Reductor<T> for $type<Option<T>>
        where
            T: cmp::Ord,
        {
            #[inline]
            fn new(v: T) -> Self {
                Self(Some(v))
            }

            #[inline]
            fn reduce(acc: Self, elem: T) -> Self {
                Self(Some(match acc.0 {
                    Some(max) => $cmp(max, elem),
                    None => elem,
                }))
            }
        }

        impl<T> Default for $type<Option<T>> {
            #[inline]
            fn default() -> Self {
                Self(None)
            }
        }
    };
}

impl_min_max!(Max, std::cmp::max);
impl_min_max!(Min, std::cmp::min);
