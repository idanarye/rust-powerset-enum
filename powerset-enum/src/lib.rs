//! The [`#[powerset_enum]`](../powerset_enum_attr/attr.powerset_enum.html) attribute parametrizes an `enum` to make it a powerset (set of all
//! subsets), and create a macro with the same name of the `enum` for easy notation of the subsets.
//!
//! Each variant of the `enum` decorated by `#[powerset_enum]` must be a tuple-struct variant with
//! a single item, and the type of that item must be unique within that `enum`.  Parametrization of
//! the `enum` (beside the one created by `#[powerset_enum]`) is not supported.
//!
//! To use a specific parametrization, use a macro with the same name of the enum and provide to it
//! the list of types you require.
//!
//! An `upcast` method is created on the `enum` type to convert any subset to any superset of that
//! subsets. Usually used with [Result::map_err].
//!
//! ```ignore
//! fn foo(...) -> Result<..., E![A, B]> {
//!     ...
//! }
//! fn bar(...) -> Result<..., E![A, B, C, D]> {
//!     foo(...).map_err(E::upcast)
//! }
//! ```
//!
//! The [Extract] `trait` provides an `extract` method on the `enum` type and on [Result] with the
//! `enum` as their error to extract a new [Result] where the OK value is the original value
//! without the extracted variant and the error is the extracted variant:
//!
//! ```ignore
//! fn baz(...) -> Result<..., E![A, B, D]> {
//!     bar(...).extract::<C>().expect("C is not allowed at all")
//! }
//! ```
//!
//! ```
//! #![feature(never_type, exhaustive_patterns, proc_macro_hygiene)]
//! # use powerset_enum::*;
//!
//! # #[derive(Debug, PartialEq)]
//! pub struct Exception1;
//! # #[derive(Debug, PartialEq)]
//! pub struct Exception2;
//! # #[derive(Debug, PartialEq)]
//! pub struct Exception3;
//!
//! #[powerset_enum]
//! # #[derive(Debug, PartialEq)]
//! pub enum Error {
//!     Exception1(Exception1),
//!     Exception2(Exception2),
//!     Exception3(Exception3),
//! }
//!
//! fn foo(x: usize) -> Result<usize, Error![Exception1, Exception2]> {
//!     Ok(match x {
//!         1 => Err(Exception1)?,
//!         2 => Err(Exception2)?,
//!         x => x,
//!     })
//! }
//!
//! fn bar(x: usize) -> Result<usize, Error![Exception1, Exception3]> {
//!     if x == 3 {
//!         Err(Exception3)?;
//!     }
//!     foo(x)
//!         // Specifically handle `Exception2`:
//!         .extract::<Exception2>().unwrap_or(Ok(2))
//!         // Convert `Result<usize, Error![Exception1]>` to `Result<usize, Error![Exception1, Exception3]>`:
//!         .map_err(Error::upcast)
//! }
//!
//! fn main() {
//!     let x = 1;
//!
//!     match foo(x) {
//!         Ok(n) => println!("OK - got {}", n),
//!         Err(Error::Exception1(_)) => println!("Got exception 1"),
//!         Err(Error::Exception2(_)) => println!("Got exception 2"),
//!         // No Exception3 match arm needed - `foo` cannot return it
//!     }
//!
//!     match bar(x) {
//!         Ok(n) => println!("OK - got {}", n),
//!         Err(Error::Exception1(_)) => println!("Got exception 1"),
//!         // No Exception2 match arm needed - `bar` cannot return it
//!         Err(Error::Exception3(_)) => println!("Got exception 3"),
//!     }
//! }
//! ```

pub use powerset_enum_attr::{powerset_enum, powerset};
pub use powerset_enum_traits::*;
