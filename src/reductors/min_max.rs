use std::cmp;

use crate::Reductor;

macro_rules! impl_min_max {
    ($type:ident, $cmp:path, $doc:expr, $similar:expr) => {
        #[doc = "Reductor that retains "]
        #[doc = $doc]
        #[doc = " value yielded by iterator"]
        #[doc = " (similary to [`"]
        #[doc = $similar]
        #[doc = "`])."]
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
    };
}

impl_min_max!(Max, std::cmp::max, "maximum", "Iterator::max");
impl_min_max!(Min, std::cmp::min, "minimum", "Iterator::min");

macro_rules! impl_float_min_max {
    ($type:ident, $float:ident, $cmp:path, $doc:expr, $cmp_name:expr) => {
        #[doc = "Reductor that retains "]
        #[doc = $doc]
        #[doc = " yielded by iterator"]
        #[doc = " (using [`"]
        #[doc = $cmp_name]
        #[doc = "`] under the hood)."]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy)]
        pub struct $type(pub $float);

        impl Reductor<$float> for $type {
            #[inline]
            fn new(v: $float) -> Self {
                Self(v)
            }

            #[inline]
            fn reduce(acc: Self, elem: $float) -> Self {
                Self($cmp(acc.0, elem))
            }
        }
    };
}

impl_float_min_max!(MaxF32, f32, f32::max, "maximum [`f32`]", "f32::max");
impl_float_min_max!(MinF32, f32, f32::min, "minimum [`f32`]", "f32::min");

impl_float_min_max!(MaxF64, f64, f64::max, "maximum [`f64`]", "f64::max");
impl_float_min_max!(MinF64, f64, f64::min, "minimum [`f64`]", "f64::min");
