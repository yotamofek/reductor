use std::ops;

use num_traits::One;

use crate::Reductor;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Product<T>(pub T);

impl<T> Default for Product<T>
where
    T: One,
{
    #[inline]
    fn default() -> Self {
        Self(<T as One>::one())
    }
}

impl<T, A> Reductor<A> for Product<T>
where
    T: ops::Mul<A, Output = T> + From<A>,
{
    #[inline]
    fn reduce(acc: Self, elem: A) -> Self {
        Self(acc.0 * elem)
    }

    fn new(item: A) -> Self {
        Self(item.into())
    }
}
