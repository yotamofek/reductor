use std::cmp;

use crate::Reductor;

#[derive(Debug, Clone, Copy)]
pub struct State<T>(T);

impl<T> From<T> for State<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<T> Default for State<Option<T>> {
    fn default() -> Self {
        Self(None)
    }
}

macro_rules! impl_min_max {
    ($type:ident, $cmp:path, $doc:expr, $similar:expr) => {
        #[doc = "Reductor that retains "]
        #[doc = $doc]
        #[doc = " value yielded by iterator"]
        #[doc = " (similary to [`"]
        #[doc = $similar]
        #[doc = "`])."]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type<T>(pub T);

        impl<T> Reductor<T> for $type<T>
        where
            T: cmp::Ord,
        {
            type State = State<T>;

            #[inline]
            fn new(v: T) -> Self::State {
                State(v)
            }

            #[inline]
            fn reduce(state: Self::State, item: T) -> Self::State {
                State($cmp(state.0, item))
            }

            #[inline]
            fn into_result(state: Self::State) -> Self {
                Self(state.0)
            }
        }

        impl<T> Reductor<T> for $type<Option<T>>
        where
            T: cmp::Ord,
        {
            type State = State<Option<T>>;

            #[inline]
            fn new(v: T) -> Self::State {
                State(Some(v))
            }

            #[inline]
            fn reduce(state: Self::State, item: T) -> Self::State {
                match state.0 {
                    Some(state) => State(Some($cmp(state, item))),
                    None => Self::new(item),
                }
            }

            #[inline]
            fn into_result(state: Self::State) -> Self {
                Self(state.0)
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
            type State = State<$float>;

            #[inline]
            fn new(v: $float) -> Self::State {
                State(v)
            }

            #[inline]
            fn reduce(state: Self::State, item: $float) -> Self::State {
                State($cmp(state.0, item))
            }

            #[inline]
            fn into_result(state: Self::State) -> Self {
                Self(state.0)
            }
        }
    };
}

impl_float_min_max!(MaxF32, f32, f32::max, "maximum [`f32`]", "f32::max");
impl_float_min_max!(MinF32, f32, f32::min, "minimum [`f32`]", "f32::min");

impl_float_min_max!(MaxF64, f64, f64::max, "maximum [`f64`]", "f64::max");
impl_float_min_max!(MinF64, f64, f64::min, "minimum [`f64`]", "f64::min");
