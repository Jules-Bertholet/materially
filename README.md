# `materially`

[![Build Status](https://github.com/Jules-Bertholet/materially/actions/workflows/actions.yml/badge.svg)](https://github.com/Jules-Bertholet/materially/actions)
[![API reference](https://img.shields.io/docsrs/materially)](https://docs.rs/materially/)
[![Crates.io](https://img.shields.io/crates/v/materially)](https://crates.io/crates/materially)
[![License](https://img.shields.io/crates/l/materially.svg)](https://github.com/Jules-Bertholet/materially#license)

A macro for [material implication](https://simple.wikipedia.org/wiki/Implication_(logic)).

`a => b` ("`a` implies `b`") means `!a || b`.

## Examples

```rust
use materially::implies as i;

assert!(i!(false => true));
assert!(i!(false => false));
assert!(i!(true => true));
assert!(!i!(true => false));

// Implication is right-associative
assert!(i!(false => false => false));

// let-chains style syntax is also supported
assert!(i!(let Some(a) = Some(17) => a > 3 => let None = Some(17) => false));
```
