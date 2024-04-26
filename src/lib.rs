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

/// The implication macro.
/// See the crate-level documentation for examples.
#[macro_export]
macro_rules! implies {
    (let $p:pat = $scrutinee:expr => $consequent:expr) => {
        match $scrutinee {
            $p => $consequent,
            _ => true,
        }
    };
    (let $p:pat = $scrutinee:expr => $($tail:tt)*) => {
        $crate::implies!(let $p = $scrutinee => $crate::implies!($($tail)*))
    };
    ($antecedent:expr => $consequent:expr) => {
        !$antecedent || $consequent
    };
    ($antecedent:expr => $($tail:tt)*) => {
        $crate::implies!($antecedent => $crate::implies!($($tail)*))
    };
}

#[cfg(test)]
mod tests {
    use super::implies as i;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test() {
        assert!(i!(false => true));
        assert!(i!(false => false));
        assert!(i!(true => true));
        assert!(!i!(true => false));

        assert!(i!(true => false => true => false));
        assert!(i!(false => false => false));
        assert!(!i!(i!(false => false) => false));

        for a in [false, true] {
            for b in [false, true] {
                for c in [false, true] {
                    assert_eq!(i!(a => b => c), i!((a && b) => c));
                    assert_eq!(i!(a => b => c), i!(a => i!(b => c)));
                }
            }
        }

        assert!(i!(let Some(foo) = Some(0) => foo < 3));
        assert!(!i!(let Some(foo) = Some(0) => foo > 3));
    }
}
