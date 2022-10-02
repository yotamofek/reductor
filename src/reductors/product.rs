use std::iter::{self, empty, once};

use crate::Reductor;

/// Reductor that multiplies items yielded by an iterator by one another (similarly to [`Iterator::product`]).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Product<T>(pub T);

#[derive(Debug, Clone, Copy)]
pub struct State<T>(T);

impl<T> From<T> for State<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<T> Default for State<T>
where
    T: iter::Product,
{
    fn default() -> Self {
        Self(empty::<T>().product())
    }
}

impl<T> Default for Product<T>
where
    T: iter::Product,
{
    #[inline]
    fn default() -> Self {
        Self(State::<T>::default().0)
    }
}

impl<A, T> Reductor<A> for Product<T>
where
    T: iter::Product + iter::Product<A>,
{
    type State = State<T>;

    fn new(item: A) -> Self::State {
        State(once(item).product())
    }

    fn reduce(state: Self::State, item: A) -> Self::State {
        State(once(state.0).chain(once(Self::new(item).0)).product())
    }

    fn into_result(state: Self::State) -> Self {
        Self(state.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Reduce;

    use super::*;

    #[test]
    fn test_product_borrowed() {
        let Product::<f64>(_) = [].iter().reduce_with();
    }
}
