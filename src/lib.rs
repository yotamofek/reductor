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
//! let Sum(sum) = iter.clone().reduce_with::<Sum<u32>>();
//! let Product(product) = iter.clone().reduce_with::<Product<u32>>();
//! let Count(count) = iter.clone().reduce_with();
//! assert_eq!(sum, iter.clone().sum());
//! assert_eq!(product, iter.clone().product());
//! assert_eq!(count, iter.clone().count());
//!
//! let Min(min) = iter.clone().fold_with::<Min<u32>>(0);
//! let Max(max) = iter.clone().fold_with::<Max<u32>>(0);
//! assert_eq!(min, iter.clone().next().unwrap());
//! assert_eq!(max, iter.last().unwrap());
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
//! let (Sum(sum), Product(product)) = iter
//!     .clone()
//!     .map(|x| (x, x * 2))
//!     .reduce_with::<(Sum<usize>, Product<usize>)>();
//!
//! assert_eq!(sum, iter.clone().sum());
//! assert_eq!(product, iter.map(|x| x * 2).product());
//! ```
//!
//! Another abstraction provided by this library is [`ReductorPair`], which allows
//! reducing an iterator producing a single value by a pair of [`Reductor`]s, in tandem.
//!
//! ```rust
//! use reductor::{Reduce, ReductorPair, Sum, Max};
//!
//! let iter = 0..10;
//! let ReductorPair(Max(max), Sum(sum)) = iter
//!     .clone()
//!     .map(|x| x)
//!     .reduce_with::<Option<ReductorPair<Max<usize>, Sum<usize>>>>().unwrap();
//!
//! assert_eq!(sum, iter.clone().sum());
//! assert_eq!(max, iter.clone().max().unwrap());
//! ```
//!
//! These constructs allow building very complex iterator loops that compose
//! numerous reductions into a single set of results.
//! ```rust
//! use reductor::{Reduce, ReductorPair, Count, Sum, Product, Max, Min};
//!
//! let iter = (0_i32..100).filter_map(|x| {
//!     if x % 2 == 0 {
//!         None
//!     } else {
//!         Some((x, x.leading_zeros()))
//!     }
//! });
//!
//! let ReductorPair(Count(count), (Sum(sum), ReductorPair(Min(min), Max(max)))) = iter
//!     .clone()
//!     .reduce_with::<Option<ReductorPair<Count, (Sum<i32>, ReductorPair<Min<u32>, Max<u32>>)>>>().unwrap();
//!
//! assert_eq!(count, iter.clone().count());
//! assert_eq!(sum, iter.clone().map(|(x, ..)| x).sum());
//! assert_eq!(min, iter.clone().map(|(.., x)| x).min().unwrap());
//! assert_eq!(max, iter.map(|(.., x)| x).max().unwrap());
//! ```

#![warn(missing_docs)]

mod reductor;
pub use self::reductor::{Reductor, ReductorPair};

mod iter;
pub use self::iter::Reduce;

pub mod reductors;
pub use reductors::*;
