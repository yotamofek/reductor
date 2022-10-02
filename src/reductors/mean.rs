use super::state::NonEmptyState;
use crate::Reductor;

/// Reductor that computes the [arithmetic mean] of items yielded by an iterator.
///
/// The generic type `F` must be one of [`f32`] or [`f64`], but the iterator's item type
/// can be any type that implements [`Into<F>`], e.g. it is possible to compute a mean
/// of type `f32` from an iterator yielding `i16`s.
///
/// [arithmetic mean]: https://en.wikipedia.org/wiki/Arithmetic_mean
///
/// # Examples
/// ```rust
/// use reductor::{Reduce, Mean};
///
/// let Mean::<f32>(mean) = [2i16, -1, -23, 42, 13]
///     .into_iter()
///     .reduce_with::<Option<_>>()
///     .unwrap();
/// assert!((mean - 6.6).abs() < f32::EPSILON);
/// ```
#[repr(transparent)]
#[allow(clippy::derive_partial_eq_without_eq)] // `F` never impls `Eq`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mean<F>(pub F);

macro_rules! impl_mean {
    ($f:ty) => {
        impl<T> Reductor<T> for Mean<$f>
        where
            T: Into<$f>,
        {
            type State = NonEmptyState<($f, usize)>;

            #[inline]
            fn new(item: T) -> Self::State {
                NonEmptyState((item.into(), 1))
            }

            #[inline]
            fn reduce(NonEmptyState((mean, count)): Self::State, item: T) -> Self::State {
                NonEmptyState((
                    mean.mul_add(count as $f, item.into()) / (count + 1) as $f,
                    count + 1,
                ))
            }

            #[inline]
            fn into_result(NonEmptyState((mean, _)): Self::State) -> Self {
                Self(mean)
            }
        }
    };
}

impl_mean!(f32);
impl_mean!(f64);

#[cfg(test)]
mod tests {
    use crate::Reduce;

    use super::*;

    #[test]
    fn test_mean() {
        macro_rules! test {
            ($f:ty) => {
                let Mean::<$f>(mean) = [0.48, 3., 2.64]
                    .into_iter()
                    .reduce_with::<Option<_>>()
                    .unwrap();
                assert!((mean - 2.04).abs() < <$f>::EPSILON);
            };
        }

        test!(f32);
        test!(f64);
    }
}
