//! A macro for [material implication](https://simple.wikipedia.org/wiki/Implication_(logic)).
//!
//! `a => b` ("`a` implies `b`") means `!a || b`.
//!
//! # Examples
//!
//! ```rust
//! use materially::implies as i;
//!
//! assert!(i!(false => true));
//! assert!(i!(false => false));
//! assert!(i!(true => true));
//! assert!(!i!(true => false));
//!
//! // Implication is right-associative
//! assert!(i!(false => false => false));
//!
//! // let-chains style syntax is also supported
//! assert!(i!(let Some(a) = Some(17) => a > 3 => let None = Some(17) => false));
//! ```

#![warn(missing_docs)]
#![no_std]

/// The implication macro.
/// See the [crate-level documentation][crate] for examples.
#[macro_export]
macro_rules! implies {
    ($($tt:tt)*) => {
        $crate::implies_tt!(() $($tt)*)
    };
}

#[doc(hidden)]
pub use core::compile_error;

#[doc(hidden)]
#[macro_export]
macro_rules! implies_tt {
    (($($front:tt)*) => $($tail:tt)*) => {
        if $($front)* {
            $crate::implies_tt_tail!(() $($tail)*)
        } else {
            true
        }
    };

    (($($front:tt)*) $next:tt $($tail:tt)*) => {
        $crate::implies_tt!(($($front)* $next) $($tail)*)
    };

    (($($front:tt)*)) => {
        {
            $crate::compile_error!("`implies` macro invocation must contain at least one implication (`=>`)");
            $($front)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! implies_tt_tail {
    (($($front:tt)*) => $($tail:tt)*) => {
        if $($front)* {
            $crate::implies_tt_tail!(() $($tail)*)
        } else {
            true
        }
    };

    (($($front:tt)*) $next:tt $($tail:tt)*) => {
        $crate::implies_tt_tail!(($($front)* $next) $($tail)*)
    };

    (($($front:tt)*)) => {
        $($front)*
    };
}
