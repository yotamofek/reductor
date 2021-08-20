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
}

impl<I> Reduce for I where I: Iterator {}
