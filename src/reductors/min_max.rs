use std::{
    clone::Clone,
    cmp::{self, Ord},
};

use crate::{Reductor, Reductors};

/// Wrapper around `T` that pointedly does NOT implement [`Default`], even though `T` might.
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

macro_rules! impl_min_max_inner {
    ($inner:ident, $cmp:path) => {
        type State = State<$inner>;

        #[inline]
        fn new(v: $inner) -> Self::State {
            State(v)
        }

        #[inline]
        fn reduce(state: Self::State, item: $inner) -> Self::State {
            State($cmp(state.0, item))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

impl<A> Reductor<A> for MinMax<A>
where
    A: Clone + Ord,
{
    minmax_impl_reductor!(A, Min: Min, Max: Max, <A>);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMaxF32 {
    pub min: f32,
    pub max: f32,
}

impl Reductor<f32> for MinMaxF32 {
    minmax_impl_reductor!(f32, Min: MinF32, Max: MaxF32);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMaxF64 {
    pub min: f64,
    pub max: f64,
}

impl Reductor<f64> for MinMaxF64 {
    minmax_impl_reductor!(f64, Min: MinF64, Max: MaxF64);
}
