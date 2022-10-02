use std::{
    clone::Clone,
    cmp::{self, Ord},
};

use super::state::NonEmptyState;
use crate::{Reductor, Reductors};

macro_rules! impl_min_max {
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

macro_rules! impl_min_max_option {
    ($inner:ident, $cmp:path) => {
        type State = NonEmptyState<Option<$inner>>;

        #[inline]
        fn new(v: $inner) -> Self::State {
            NonEmptyState(Some(v))
        }

        #[inline]
        fn reduce(state: Self::State, item: $inner) -> Self::State {
            match state.0 {
                Some(state) => NonEmptyState(Some($cmp(state, item))),
                None => Self::new(item),
            }
        }

        #[inline]
        fn into_result(state: Self::State) -> Self {
            Self(state.0)
        }
    };
}

/// Reductor that retains the minimum value yielded by an iterator (similarly to [`Iterator::min`]).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Min<T>(pub T);

impl<T> Reductor<T> for Min<T>
where
    T: Ord,
{
    impl_min_max!(T, cmp::min);
}

impl<T> Reductor<T> for Min<Option<T>>
where
    T: Ord,
{
    impl_min_max_option!(T, cmp::min);
}

/// Reductor that retains the maximum value yielded by an iterator (similarly to [`Iterator::max`]).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Max<T>(pub T);

impl<T> Reductor<T> for Max<T>
where
    T: Ord,
{
    impl_min_max!(T, cmp::max);
}

impl<T> Reductor<T> for Max<Option<T>>
where
    T: Ord,
{
    impl_min_max_option!(T, cmp::max);
}

/// Reductor that retains the maximum float value yielded by an iterator (similarly to [`Iterator::max`],
/// but using [`f64::max`] or [`f32::max`] under the hood).
#[repr(transparent)]
#[allow(clippy::derive_partial_eq_without_eq)] // `F` never impls `Eq`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxF<F>(pub F);

impl Reductor<f32> for MaxF<f32> {
    impl_min_max!(f32, f32::max);
}

impl Reductor<f32> for MaxF<Option<f32>> {
    impl_min_max_option!(f32, f32::max);
}

impl Reductor<f64> for MaxF<f64> {
    impl_min_max!(f64, f64::max);
}

impl Reductor<f64> for MaxF<Option<f64>> {
    impl_min_max_option!(f64, f64::max);
}

/// Reductor that retains the minimum float value yielded by an iterator (similarly to [`Iterator::min`],
/// but using [`f64::min`] or [`f32::min`] under the hood).
#[repr(transparent)]
#[allow(clippy::derive_partial_eq_without_eq)] // `F` never impls `Eq`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MinF<F>(pub F);

impl Reductor<f32> for MinF<f32> {
    impl_min_max!(f32, f32::min);
}

impl Reductor<f32> for MinF<Option<f32>> {
    impl_min_max_option!(f32, f32::min);
}

impl Reductor<f64> for MinF<f64> {
    impl_min_max!(f64, f64::min);
}

impl Reductor<f64> for MinF<Option<f64>> {
    impl_min_max_option!(f64, f64::min);
}

macro_rules! minmax_impl_reductor {
    ($type:ident, Min: $min:ident, Max: $max:ident) => {
        minmax_impl_reductor!(
            $type,
            Min: $min,
            Max: $max,
            Pair: Reductors<($min<$type>, $max<$type>)>
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
            let Reductors(($min(min), $max(max))) =
                <$pair_type as Reductor<$type>>::into_result(state);

            Self { min, max }
        }
    };
}

/// Reductor that retains both the minimum and the maximum values yielded by an iterator.
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
    minmax_impl_reductor!(A, Min: Min, Max: Max);
}

/// Reductor that retains both the minimum and the maximum float values yielded by an iterator
/// (using [`f64::min`] and [`f64::max`], or [`f32::min`] and [`f32::max`] under the hood).
#[allow(clippy::derive_partial_eq_without_eq)] // `F` never impls `Eq`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMaxF<F> {
    /// Minimum value yielded by iterator.
    pub min: F,
    /// Maximum value yielded by iterator.
    pub max: F,
}

impl Reductor<f32> for MinMaxF<f32> {
    minmax_impl_reductor!(f32, Min: MinF, Max: MaxF);
}

impl Reductor<f64> for MinMaxF<f64> {
    minmax_impl_reductor!(f64, Min: MinF, Max: MaxF);
}
