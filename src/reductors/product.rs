use std::iter::{self, empty, once};

use crate::Reductor;

/// Reductor that multiplies items yielded by an iterator by one another (similary to [`Iterator::product`]).
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Product<T>(pub T);

impl<T> Default for Product<T>
where
    T: iter::Product,
{
    #[inline]
    fn default() -> Self {
        Self(empty::<T>().product())
    }
}

impl<T, A> Reductor<A> for Product<T>
where
    T: iter::Product + From<A>,
{
    #[inline]
    fn new(item: A) -> Self {
        Self(item.into())
    }

    #[inline]
    fn reduce(acc: Self, elem: A) -> Self {
        Self(once(acc.0).chain(once(elem.into())).product())
    }
}
