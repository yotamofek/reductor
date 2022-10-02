use std::iter::{self, empty, once};

use crate::Reductor;

/// Reductor that adds items yielded by an iterator to each other (similarly to [`Iterator::sum`]).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sum<T>(pub T);

impl<T> Default for Sum<T>
where
    T: iter::Sum,
{
    #[inline]
    fn default() -> Self {
        Self(empty::<T>().sum())
    }
}

impl<A, T> Reductor<A> for Sum<T>
where
    T: iter::Sum + iter::Sum<A>,
{
    type State = T;

    fn new(item: A) -> Self::State {
        once(item).sum()
    }

    fn reduce(state: Self::State, item: A) -> Self::State {
        once(state).chain(once(Self::new(item))).sum()
    }

    fn into_result(state: Self::State) -> Self {
        Self(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::Reduce;

    use super::*;

    #[test]
    fn test_sum_borrowed() {
        let Sum::<f64>(_) = [].iter().reduce_with();
    }
}
