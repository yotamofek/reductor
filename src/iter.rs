use super::Reductor;

/// Allow reducing an [`Iterator`] with a [`Reductor`].
pub trait Reduce: Iterator + Sized {
    /// Similar to [`Iterator::reduce`], but uses a generic implementation of [`Reductor`],
    /// instead of a function parameter, to supply the reduction logic.
    #[inline]
    fn reduce_with<R>(self) -> R
    where
        R: Reductor<Self::Item> + Default,
    {
        let reductor = R::default();
        self.fold(reductor, <R as Reductor<Self::Item>>::reduce)
    }

    /// Similar to [`Iterator::fold`], but uses a generic implementation of [`Reductor`],
    /// instead of a function parameter, to supply the reduction logic.
    #[inline]
    fn fold_with<R>(self, init: Self::Item) -> R
    where
        R: Reductor<Self::Item>,
    {
        let reductor = <R as Reductor<Self::Item>>::new(init);
        self.fold(reductor, <R as Reductor<Self::Item>>::reduce)
    }
}

impl<I> Reduce for I where I: Iterator {}
