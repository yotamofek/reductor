use std::{convert::TryInto, num::NonZeroUsize};

use crate::Reductor;

/// Reductor that counts the number of items yielded by an iterator (similary to [`Iterator::count`]).
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(pub usize);

impl<A> Reductor<A> for Count {
    type State = usize;

    fn new(_: A) -> Self::State {
        1
    }

    fn reduce(state: Self::State, _: A) -> Self::State {
        state + 1
    }

    fn into_result(state: Self::State) -> Self {
        Self(state)
    }
}

/// ```rust
/// use std::num::NonZeroUsize;
///
/// use reductor::{Reduce, ReductorPair, CountNonZero, Max};
///
/// let iter = (0..100).filter(|&i| i % 2 == 0);
///
/// let ReductorPair(CountNonZero(count), Max::<usize>(max)) = iter.clone()
///     .reduce_with::<Option<_>>()
///     .unwrap();
///
/// assert_eq!(count, NonZeroUsize::new(50).unwrap());
/// assert_eq!(max, iter.last().unwrap());
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountNonZero(pub NonZeroUsize);

impl<A> Reductor<A> for CountNonZero {
    type State = NonZeroUsize;

    fn new(_: A) -> Self::State {
        NonZeroUsize::new(1).unwrap()
    }

    fn reduce(state: Self::State, _: A) -> Self::State {
        (state.get() + 1).try_into().unwrap()
    }

    fn into_result(state: Self::State) -> Self {
        Self(state)
    }
}
