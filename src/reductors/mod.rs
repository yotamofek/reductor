//! Standard implementations of [`Reductor`](crate::Reductor).

mod count;
pub use self::count::{Count, CountNonZero};

mod sum;
pub use self::sum::Sum;

mod product;
pub use self::product::Product;

mod min_max;
pub use self::min_max::{Max, MaxF32, MaxF64, Min, MinF32, MinF64, MinMax, MinMaxF32, MinMaxF64};
