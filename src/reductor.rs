/// Reductors are types that implement the logic for [`fold`](Iterator::fold)ing an iterator
/// into a single result.
///
/// ```rust
/// use reductor::{Reductor, Reduce};
///
/// #[derive(Default)]
/// struct Mean { mean: f32, count: usize };
///
/// impl Reductor<f32> for Mean {
///     fn new(item: f32) -> Self {
///         Self { mean: item, count: 1 }
///     }
///
///     fn reduce(acc: Self, item: f32) -> Self {
///         let mean = acc.mean * (acc.count as f32);
///         let count = acc.count + 1;
///         Self {
///             mean: (mean + item) / (count as f32),
///             count,
///         }
///     }
/// }
///
/// let Mean { mean, .. } = vec![8.5, -5.5, 2.0, -4.0].into_iter().reduce_with::<Mean>();
/// assert!((mean - 0.25).abs() < f32::EPSILON, "Wrong mean: {}", mean);
/// ```
///
/// # Blanket implementations
///
/// ## Option
/// Since `Reductor` is implemented for `Option<R> where R: Reducer`, a reductor that
/// does not implement the [`Default`] trait (e.g. [`Max`](crate::reductors::Max) and
/// [`Min`](crate::reductors::Min)) can be used with [`reduce_with`](crate::Reduce::reduce_with)
/// by wrapping it in an [`Option`].
///
/// ## Two-element tuple
/// Two `Reductor`s can be combined in a tuple to reduce iterators that product two-element tuples.
///
/// ```rust
/// use reductor::{Reduce, Sum, Product};
///
/// let iter1 = (50..60);
/// let iter2 = (10..20);
/// let (Sum(sum), Product(product)) = iter1.clone().zip(iter2.clone())
///     .reduce_with::<(Sum<u64>, Product<u64>)>();
///
/// assert_eq!(sum, iter1.sum());
/// assert_eq!(product, iter2.product())
/// ```

pub trait Reductor<A>: Sized {
    /// This method will be called with the first item yielded by an iterator
    /// to create the initial state of the reductor.
    fn new(item: A) -> Self;

    /// Reduce the current accumulated state with the next item yielded by an iterator,
    /// returning the new state.
    fn reduce(acc: Self, item: A) -> Self;
}

impl<R, A> Reductor<A> for Option<R>
where
    R: Reductor<A>,
{
    #[inline]
    fn new(item: A) -> Self {
        Some(<R as Reductor<A>>::new(item))
    }

    #[inline]
    fn reduce(acc: Self, item: A) -> Self {
        Some(match acc {
            Some(acc) => <R as Reductor<A>>::reduce(acc, item),
            None => <R as Reductor<A>>::new(item),
        })
    }
}

impl<R1, R2, A1, A2> Reductor<(A1, A2)> for (R1, R2)
where
    R1: Reductor<A1>,
    R2: Reductor<A2>,
{
    #[inline]
    fn new((item1, item2): (A1, A2)) -> Self {
        (
            <R1 as Reductor<A1>>::new(item1),
            <R2 as Reductor<A2>>::new(item2),
        )
    }

    #[inline]
    fn reduce(acc: Self, (item1, item2): (A1, A2)) -> Self {
        (
            <R1 as Reductor<A1>>::reduce(acc.0, item1),
            <R2 as Reductor<A2>>::reduce(acc.1, item2),
        )
    }
}

/// This struct can be used to pair two [`Reductor`]s to run on a single value,
/// by [cloning](`Clone`) every element yielded, and updating both `Reductor`s'
/// states.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductorPair<R1, R2>(pub R1, pub R2);

impl<R1, R2, A> Reductor<A> for ReductorPair<R1, R2>
where
    A: Clone,
    R1: Reductor<A>,
    R2: Reductor<A>,
{
    #[inline]
    fn new(item: A) -> Self {
        Self(
            <R1 as Reductor<A>>::new(item.clone()),
            <R2 as Reductor<A>>::new(item),
        )
    }

    #[inline]
    fn reduce(acc: Self, item: A) -> Self {
        Self(
            <R1 as Reductor<A>>::reduce(acc.0, item.clone()),
            <R2 as Reductor<A>>::reduce(acc.1, item),
        )
    }
}
