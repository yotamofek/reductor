pub trait Reductor<A>: Sized {
    fn new(item: A) -> Self;
    fn reduce(acc: Self, item: A) -> Self;
}

impl<R, A> Reductor<A> for Option<R>
where
    R: Reductor<A>,
{
    fn reduce(acc: Self, item: A) -> Self {
        Some(match acc {
            Some(acc) => <R as Reductor<A>>::reduce(acc, item),
            None => <R as Reductor<A>>::new(item),
        })
    }

    fn new(item: A) -> Self {
        Some(<R as Reductor<A>>::new(item))
    }
}

impl<R1, R2, A1, A2> Reductor<(A1, A2)> for (R1, R2)
where
    R1: Reductor<A1>,
    R2: Reductor<A2>,
{
    #[inline]
    fn reduce(acc: Self, (item1, item2): (A1, A2)) -> Self {
        (
            <R1 as Reductor<A1>>::reduce(acc.0, item1),
            <R2 as Reductor<A2>>::reduce(acc.1, item2),
        )
    }

    fn new((item1, item2): (A1, A2)) -> Self {
        (
            <R1 as Reductor<A1>>::new(item1),
            <R2 as Reductor<A2>>::new(item2),
        )
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductorPair<R1, R2>(pub R1, pub R2);

impl<R1, R2, A> Reductor<A> for ReductorPair<R1, R2>
where
    A: Clone,
    R1: Reductor<A>,
    R2: Reductor<A>,
{
    #[inline]
    fn reduce(acc: Self, item: A) -> Self {
        Self(
            <R1 as Reductor<A>>::reduce(acc.0, item.clone()),
            <R2 as Reductor<A>>::reduce(acc.1, item),
        )
    }

    fn new(item: A) -> Self {
        Self(
            <R1 as Reductor<A>>::new(item.clone()),
            <R2 as Reductor<A>>::new(item),
        )
    }
}
