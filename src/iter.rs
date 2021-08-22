use super::Reductor;

/// Allow reducing an [`Iterator`] with a [`Reductor`].
pub trait Reduce: Iterator + Sized {
    /// Similar to [`Iterator::reduce`], but uses a generic implementation of [`Reductor`],
    /// instead of a function parameter, to supply the reduction logic.
    #[inline]
    fn reduce_with<R>(self) -> R
    where
        R: Reductor<Self::Item>,
        R::State: Default,
    {
        let state = R::State::default();
        R::into_result(self.fold(state, R::reduce))
    }

    /// Similar to [`Iterator::fold`], but uses a generic implementation of [`Reductor`],
    /// instead of a function parameter, to supply the reduction logic.
    #[inline]
    fn fold_with<R, I>(self, init: I) -> R
    where
        R: Reductor<Self::Item>,
        R::State: From<I>,
    {
        R::into_result(self.fold(init.into(), R::reduce))
    }
}

impl<I> Reduce for I where I: Iterator {}
