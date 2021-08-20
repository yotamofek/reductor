use super::Reductor;

pub trait Reduce: Iterator + Sized {
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
