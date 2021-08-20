use crate::Reductor;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(pub usize);

impl<A> Reductor<A> for Count {
    #[inline]
    fn new(_: A) -> Self {
        Self(1)
    }

    #[inline]
    fn reduce(acc: Self, _: A) -> Self {
        Self(acc.0 + 1)
    }
}
