/// Wrapper around `T` that pointedly does NOT implement [`Default`], even though `T` might.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct NonEmptyState<T>(pub(super) T);

impl<T> From<T> for NonEmptyState<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<T> Default for NonEmptyState<Option<T>> {
    fn default() -> Self {
        Self(None)
    }
}
