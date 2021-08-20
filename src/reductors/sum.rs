use std::iter::{self, empty, once};

use crate::Reductor;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Sum<T>(pub T);

impl<T> From<T> for Sum<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> Default for Sum<T>
where
    T: iter::Sum,
{
    #[inline]
    fn default() -> Self {
        Self(empty::<T>().sum())
    }
}

impl<T, A> Reductor<A> for Sum<T>
where
    T: iter::Sum + From<A>,
{
    #[inline]
    fn reduce(acc: Self, elem: A) -> Self {
        Self(once(acc.0).chain(once(elem.into())).sum())
    }

    #[inline]
    fn new(item: A) -> Self {
        Self(item.into())
    }
}
