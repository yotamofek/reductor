use std::{
    clone::Clone,
    cmp::{self, Ord},
};

use super::state::NonEmptyState;
use crate::{Reductor, Reductors};

macro_rules! impl_min_max_inner {
    ($inner:ident, $cmp:path) => {
        type State = NonEmptyState<$inner>;

        #[inline]
        fn new(v: $inner) -> Self::State {
            NonEmptyState(v)
        }

        #[inline]
        fn reduce(state: Self::State, item: $inner) -> Self::State {
            NonEmptyState($cmp(state.0, item))
        }

        #[inline]
        fn into_result(state: Self::State) -> Self {
            Self(state.0)
        }
    };
}

macro_rules! impl_min_max {
    ($type:ident, $cmp:path, $doc:expr) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type<T>(pub T);

        impl<T> Reductor<T> for $type<T>
        where
            T: Ord,
        {
            impl_min_max_inner!(T, $cmp);
        }

        impl<T> Reductor<T> for $type<Option<T>>
        where
            T: Ord,
        {
            type State = NonEmptyState<Option<T>>;

            #[inline]
            fn new(v: T) -> Self::State {
                NonEmptyState(Some(v))
            }

            #[inline]
            fn reduce(state: Self::State, item: T) -> Self::State {
                match state.0 {
                    Some(state) => NonEmptyState(Some($cmp(state, item))),
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

macro_rules! min_max_doc_inner {
    ($value:expr, $parens:expr) => {
        concat!(
            "Reductor that retains ",
            $value,
            " value yielded by iterator (",
            $parens,
            ")."
        )
    };
}

macro_rules! min_max_doc {
    ($value:expr, $cmp:ident) => {
        min_max_doc_inner!(
            $value,
            concat!("similarly to [`Iterator::", stringify!($cmp), "`]")
        )
    };
}

impl_min_max!(Min, cmp::min, min_max_doc!("minimum", min));
impl_min_max!(Max, cmp::max, min_max_doc!("maximum", max));

macro_rules! impl_float_min_max_inner {
    ($type:ident, $float:ident, $cmp:ident, $doc:expr) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $type(pub $float);

        impl Reductor<$float> for $type {
            impl_min_max_inner!($float, $float::$cmp);
        }
    };
}

macro_rules! float_min_max_doc {
    ($type:ident, $value:expr, $cmp:ident) => {
        min_max_doc_inner!(
            $value,
            concat!(
                "using [`",
                stringify!($type),
                "::",
                stringify!($cmp),
                "`] under the hood"
            )
        )
    };
}

macro_rules! impl_float_min_max {
    (
        $type:ident,
        Min: $min_type:ident,
        Max: $max_type:ident
    ) => {
        impl_float_min_max_inner!(
            $min_type,
            $type,
            min,
            float_min_max_doc!($type, "minimum", min)
        );
        impl_float_min_max_inner!(
            $max_type,
            $type,
            max,
            float_min_max_doc!($type, "maximum", max)
        );
    };
}

impl_float_min_max!(f32, Min: MinF32, Max: MaxF32);
impl_float_min_max!(f64, Min: MinF64, Max: MaxF64);

macro_rules! minmax_impl_reductor {
    ($type:ident, Min: $min:ident, Max: $max:ident $(, <$generic:ident>)?) => {
        minmax_impl_reductor!(
            $type,
            Min: $min,
            Max: $max,
            Pair: Reductors<($min$(<$generic>)?, $max$(<$generic>)?)>
        );
    };
    ($type:ident, Min: $min:ident, Max: $max:ident, Pair: $pair_type:ty) => {
        type State = <$pair_type as Reductor<$type>>::State;

        fn new(item: $type) -> Self::State {
            <$pair_type as Reductor<$type>>::new(item)
        }

        fn reduce(state: Self::State, item: $type) -> Self::State {
            <$pair_type as Reductor<$type>>::reduce(state, item)
        }

        fn into_result(state: Self::State) -> Self {
            let Reductors(($min(min), $max(max))) = <$pair_type as Reductor<$type>>::into_result(state);

            Self { min, max }
        }
    };
}

/// Reductor that retains both the minimum and maximum values yielded by iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinMax<T> {
    /// Minimum value yielded by iterator.
    pub min: T,
    /// Maximum value yielded by iterator.
    pub max: T,
}

impl<A> Reductor<A> for MinMax<A>
where
    A: Clone + Ord,
{
    minmax_impl_reductor!(A, Min: Min, Max: Max, <A>);
}

/// Reductor that retains both the minimum and maximum values yielded by iterator over [`f32`]s (using [`f32::min`] and [`f32::max`] under the hood).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMaxF32 {
    /// Minimum value yielded by iterator.
    pub min: f32,
    /// Maximum value yielded by iterator.
    pub max: f32,
}

impl Reductor<f32> for MinMaxF32 {
    minmax_impl_reductor!(f32, Min: MinF32, Max: MaxF32);
}

/// Reductor that retains both the minimum and maximum values yielded by iterator over [`f64`]s (using [`f64::min`] and [`f64::max`] under the hood).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMaxF64 {
    /// Minimum value yielded by iterator.
    pub min: f64,
    /// Maximum value yielded by iterator.
    pub max: f64,
}

impl Reductor<f64> for MinMaxF64 {
    minmax_impl_reductor!(f64, Min: MinF64, Max: MaxF64);
}
