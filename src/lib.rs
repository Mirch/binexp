//! A library for working with power of two numbers.
//! Using the `PowerOfTwo` struct can be useful for validation purposes, when you want to make sure that a number is a power of two.
//! `PowerOfTwo` implements `TryFrom` for all integer types, so you can convert any integer into a `PowerOfTwo` instance.
//!
//! <br>
//!
//! # Examples
//! ```
//! use std::convert::TryFrom;
//! use binexp::PowerOfTwo;
//!
//! assert!(PowerOfTwo::try_from(1).is_ok());
//! assert!(PowerOfTwo::try_from(2).is_ok());
//! assert!(PowerOfTwo::try_from(3).is_err());
//!
//! let four = PowerOfTwo::try_from(4).unwrap();
//! assert_eq!(four.to_string(), "2^2 (4)");
//!
//! let eigth: PowerOfTwo = 8i8.try_into().unwrap();
//! assert_eq!(eigth.to_string(), "2^3 (8)");
//! ```

pub mod errors;
pub mod power_of_two;

pub use errors::PowerOfTwoError;
pub use power_of_two::PowerOfTwo;
