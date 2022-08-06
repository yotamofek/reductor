//! Generic abstractions for combining and nesting reduction patterns for iterables.
//!
//! The main entry points to this library are [`Reduce::reduce_with`] and
//! [`Reduce::fold_with`], which can be called on any [`Iterator`], and are
//! similar to [`Iterator::reduce`] and [`Iterator::fold`], respectively,
//! but use a generic implementation of a [`Reductor`] for the reduction logic.
//!
//! The following examples shows some of the basic building blocks from which `reductor`
//! enables building more complex patterns:
//!
//! ```rust
//! use reductor::{Reduce, Sum, Product, Count, Min, Max};
//!
//! let iter = 0..10;
//!
//! let Sum::<u32>(sum) = iter.clone().reduce_with();
//! let Product::<u32>(product) = iter.clone().fold_with(2);
//! let Count(count) = iter.clone().reduce_with();
//! assert_eq!(sum, iter.clone().sum());
//! assert_eq!(product, iter.clone().product::<u32>() * 2);
//! assert_eq!(count, iter.clone().count());
//!
//! let min: Option<Min<u32>> = iter.clone().reduce_with();
//! let Max::<Option<u32>>(max) = iter.clone().reduce_with();
//! assert_eq!(min.unwrap(), Min(iter.start));
//! assert_eq!(max.unwrap(), iter.last().unwrap());
//! ```
//!
//! Notice that unlike [`Sum`] and [`Product`], [`Min`] and [`Max`] won't reduce
//! an empty iterator into the default value. This mirrors the way [`Iterator::max`]
//! returns an [`Option<T>`], unlike [`Iterator::sum`].
//!
//!
//! Now, let's combine two [`Reductor`]s to reduce an iterator that produces a pair of values:
//!
//! ```rust
//! use reductor::{Reduce, Sum, Product};
//!
//! let iter = 0..10;
//!
//! let (Sum::<usize>(sum), Product::<usize>(product)) = iter
//!     .clone()
//!     .map(|x| (x, x * 2))
//!     .reduce_with();
//!
//! assert_eq!(sum, iter.clone().sum());
//! assert_eq!(product, iter.map(|x| x * 2).product());
//! ```
//!
//! Another abstraction provided by this library is [`Reductors`], which allows
//! reducing an iterator producing a single value by a tuple of [`Reductor`]s, in tandem.
//!
//! ```rust
//! use reductor::{Reduce, Reductors, Min, Max, Sum};
//!
//! let iter = 0..10;
//!
//! let Reductors((Min::<usize>(min), Max::<usize>(max))) = iter
//!     .clone()
//!     .reduce_with::<Option<_>>()
//!     .unwrap();
//!
//! assert_eq!(min, iter.start);
//! assert_eq!(max, iter.end - 1);
//!
//! let Reductors((Min::<Option<usize>>(min), Sum::<usize>(sum))) = iter
//!     .clone()
//!     .reduce_with();
//!
//! assert_eq!(min.unwrap(), iter.start);
//! assert_eq!(sum, iter.sum());
//! ```
//!
//! These constructs allow building very complex iterator loops that compose
//! numerous reductions into a single set of results.
//! ```rust
//! use reductor::{Reduce, Reductors, Count, Sum, Max, Min};
//!
//! let iter = (0_i32..100).filter_map(|x| {
//!     if x % 2 == 0 {
//!         None
//!     } else {
//!         Some((x, x.leading_zeros()))
//!     }
//! });
//!
//! let Reductors((
//!     Count(count),
//!     (Sum::<i32>(sum), Reductors((Min::<u32>(min), Max::<u32>(max)))),
//! )) = iter.clone().reduce_with::<Option<_>>().unwrap();
//!
//! assert_eq!(count, iter.clone().count());
//! assert_eq!(sum, iter.clone().map(|(x, ..)| x).sum());
//! assert_eq!(min, iter.clone().map(|(.., x)| x).min().unwrap());
//! assert_eq!(max, iter.map(|(.., x)| x).max().unwrap());
//! ```

#![warn(missing_docs)]

mod reductor;
pub use self::reductor::{Reductor, Reductors};

mod iter;
pub use self::iter::Reduce;

pub mod reductors;
pub use reductors::*;
