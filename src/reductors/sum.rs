use std::ops;

use crate::Reductor;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Sum<T>(pub T);

impl<T> From<T> for Sum<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T, A> Reductor<A> for Sum<T>
where
    T: ops::Add<A, Output = T> + From<A>,
{
    #[inline]
    fn reduce(acc: Self, elem: A) -> Self {
        Self(acc.0 + elem)
    }

    #[inline]
    fn new(item: A) -> Self {
        Self(item.into())
    }
}
