use std::cmp;

use crate::Reductor;

macro_rules! impl_min_max {
    ($type:ident, $cmp:path) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy)]
        pub struct $type<T>(pub T);

        impl<T> From<T> for $type<T> {
            fn from(t: T) -> Self {
                Self(t)
            }
        }

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
            fn default() -> Self {
                Self(None)
            }
        }
    };
}

impl_min_max!(Max, std::cmp::max);
impl_min_max!(Min, std::cmp::min);

// macro_rules! impl_min_max_tuple {
//     ($type1:ident, $type2:ident, $cmp1:path, $cmp2:path) => {
//         impl<T> Reductor<T> for Option<($type1<T>, $type2<T>)>
//         where
//             T: Ord + Clone,
//         {
//             fn reduce(acc: Self, item: T) -> Self {
//                 Some(match acc {
//                     Some(($type1(min), $type2(max))) => {
//                         ($type1($cmp1(min, item.clone())), $type2($cmp2(max, item)))
//                     }
//                     None => ($type1(item.clone()), $type2(item)),
//                 })
//             }
//         }
//     };
// }

// impl_min_max_tuple!(Max, Min, std::cmp::max, std::cmp::min);
// impl_min_max_tuple!(Min, Max, std::cmp::min, std::cmp::max);
