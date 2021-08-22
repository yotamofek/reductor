/// Reductors are types that implement the logic for [`fold`](Iterator::fold)ing an iterator
/// into a single result.
///
/// ```rust
/// use reductor::{Reductor, Reduce};
///
/// struct MeanState { mean: f32, count: usize };
///
/// struct Mean(f32);
///
/// impl Reductor<f32> for Mean {
///     type State = MeanState;
///     
///     fn new(item: f32) -> Self::State {
///         MeanState { mean: item, count: 1 }
///     }
///
///     fn reduce(acc: Self::State, item: f32) -> Self::State {
///         MeanState {
///             mean: acc.mean + item,
///             count: acc.count + 1,
///         }
///     }
///     
///     fn into_result(state: Self::State) -> Self {
///         Self(state.mean / state.count as f32)
///     }
/// }
///
/// let Mean(mean) = vec![8.5, -5.5, 2.0, -4.0].into_iter()
///     .reduce_with::<Option<Mean>>()
///     .unwrap();
///
/// assert!((mean - 0.25).abs() < f32::EPSILON);
/// ```
pub trait Reductor<A>: Sized {
    /// Intermediate state for the reductor.
    ///
    /// This type is used to keep track of the state of reduction while processing
    /// an iterator. The first item yielded is converted into the `State` type by
    /// calling [`new`](Reductor::new). The next item will be reduced using the previous
    /// state by calling [`reduce`](Reductor::reduce) with the new item, and the resulting
    /// state will be used for the next reduction, and so forth. When the iterator
    /// is exhausted, the intermediate state will be turned into a result by calling
    /// [`into_result`](Reductor::into_result).
    ///
    /// `State` must implement the [`Default`] trait for the `Reductor` to be used
    /// with [`reduce_with`](crate::Reduce::reduce_with), otherwise, an initial state
    /// can be provided by calling [`fold_with`](crate::Reduce::fold_with).
    type State;

    /// This method will be called with the first item yielded by an iterator
    /// to create the initial state of the reductor.
    fn new(item: A) -> Self::State;

    /// Reduce the current accumulated state with the next item yielded by an iterator,
    /// returning the new state.
    fn reduce(state: Self::State, item: A) -> Self::State;

    /// After reducing the entire iterator, and exhausting it, turn the final state into
    /// a result.
    fn into_result(state: Self::State) -> Self;
}

/// Wrapping a [`Reductor`] in an [`Option`] allows using [`reduce_with`](crate::Reduce::reduce_with)
/// with a `Reductor` whose [`State`](Reductor::State) does not implement [`Default`].
///
/// ```compile_fail
/// # use reductor::{Reduce, Min};
/// let _ = (0..10).reduce_with::<Min<u32>>();
/// ```
///
/// ```rust
/// # use reductor::{Reduce, Min};
/// let _ = (0..10).reduce_with::<Option<Min<u32>>>();
/// ```
impl<R, A> Reductor<A> for Option<R>
where
    R: Reductor<A>,
{
    type State = Option<R::State>;

    fn new(item: A) -> Self::State {
        Some(R::new(item))
    }

    fn reduce(state: Self::State, item: A) -> Self::State {
        Some(match state {
            None => R::new(item),
            Some(state) => R::reduce(state, item),
        })
    }

    fn into_result(state: Self::State) -> Self {
        state.map(R::into_result)
    }
}

/// Two `Reductor`s can be combined in a tuple to reduce iterators that yield two-element tuples.
///
/// ```rust
/// use reductor::{Reduce, Sum, Product};
///
/// let iter = (5..10).map(|x| (x, -(x as i64)));
///
/// let (Sum(sum), Product(product)) = iter
///     .clone()
///     .reduce_with::<(Sum<u64>, Product<i64>)>();
///
/// assert_eq!(sum, iter.clone().map(|(x, ..)| x).sum());
/// assert_eq!(product, iter.clone().map(|(.., x)| x).product())
/// ```
///
/// See [`ReductorPair`] for reducing a single-item tuple with two `Reductor`s.
impl<R1, R2, A1, A2> Reductor<(A1, A2)> for (R1, R2)
where
    R1: Reductor<A1>,
    R2: Reductor<A2>,
{
    type State = (R1::State, R2::State);

    fn new(item: (A1, A2)) -> Self::State {
        (R1::new(item.0), R2::new(item.1))
    }

    fn reduce(state: Self::State, item: (A1, A2)) -> Self::State {
        (R1::reduce(state.0, item.0), R2::reduce(state.1, item.1))
    }

    fn into_result(state: Self::State) -> Self {
        (R1::into_result(state.0), R2::into_result(state.1))
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
    type State = (R1::State, R2::State);

    fn new(item: A) -> Self::State {
        (R1::new(item.clone()), R2::new(item))
    }

    fn reduce(state: Self::State, item: A) -> Self::State {
        (R1::reduce(state.0, item.clone()), R2::reduce(state.1, item))
    }

    fn into_result(state: Self::State) -> Self {
        Self(R1::into_result(state.0), R2::into_result(state.1))
    }
}
