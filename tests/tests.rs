#![allow(clippy::assertions_on_constants)]

use materially::implies as i;

#[test]
fn truth_table() {
    assert!(i!(false => true));
    assert!(i!(false => false));
    assert!(i!(true => true));
    assert!(!i!(true => false));
}

#[test]
fn associativity() {
    assert!(i!(true => false => true => false));
    assert!(i!(false => false => false));
    assert!(!i!(i!(false => false) => false));
}

#[test]
fn short_circuiting() {
    let mut a = 0;

    assert!(i!(false => { a += 1; false }));
    assert_eq!(a, 0);

    assert!(i!(true => { a += 1; true }));
    assert_eq!(a, 1);
}

#[test]
fn properties() {
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                assert_eq!(i!(a => b => c), i!(a && b => c));
                assert_eq!(i!(a => b => c), i!(a => i!(b => c)));
            }
        }
    }
}

#[allow(clippy::disallowed_names)]
#[test]
fn let_chains() {
    assert!(i!(let Some(foo) = Some(0) => foo < 3));
    assert!(!i!(let Some(foo) = Some(0) => foo > 3));
}
