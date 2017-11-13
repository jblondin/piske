extern crate piske;
extern crate sindra;

use sindra::PNode;

use piske::parse::{expression};
use piske::ast::ast::*;

include!("macros.rs");

#[test]
fn test_infix() {

    assert_eq!(expression("2 + 3"), Ok(add!(int!(2), int!(3))));
    assert_eq!(expression("2 +3"), Ok(add!(int!(2), int!(3))));
    assert_eq!(expression("2+ 3"), Ok(add!(int!(2), int!(3))));
    assert_eq!(expression("1 + 2 * 3"), Ok(add!(int!(1), multiply!(int!(2), int!(3)))));
    assert_eq!(expression("(1 + 2) * 3"), Ok(multiply!(add!(int!(1), int!(2)), int!(3))));
    assert_eq!(expression("1 - 2 + 3"), Ok(add!(subtract!(int!(1), int!(2)), int!(3))));
    assert_eq!(expression("1 * (2 + 3) / 4"),
        Ok(divide!(multiply!(int!(1), add!(int!(2), int!(3))), int!(4))));

    assert_eq!(expression("(1 + 2) * (3 + 4)"),
        Ok(multiply!(add!(int!(1), int!(2)), add!(int!(3), int!(4)))));
    assert_eq!(expression("1 + 2 * 3 + 4"),
        Ok(add!(add!(int!(1), multiply!(int!(2), int!(3))), int!(4))));

    assert_eq!(expression("(1 + 3)"), Ok(add!(int!(1), int!(3))));
    assert_eq!(expression("(1 * (2 + 3))"), Ok(multiply!(int!(1), add!(int!(2), int!(3)))));
    assert_eq!(expression("(1 * (2 + (3 - 4)))"),
        Ok(multiply!(int!(1), add!(int!(2), subtract!(int!(3), int!(4))))));

    assert_eq!(expression("1 + 2 + 3"), Ok(add!(add!(int!(1), int!(2)), int!(3))));
    assert_eq!(expression("1 + (2 + 3)"), Ok(add!(int!(1), add!(int!(2), int!(3)))));
}

#[test]
fn test_prefix() {
    assert_eq!(expression("-2"), Ok(uminus!(int!(2))));
    assert_eq!(expression(" - 2"), Ok(uminus!(int!(2))));
    assert_eq!(expression(" -2"), Ok(uminus!(int!(2))));
    assert_eq!(expression("- 2"), Ok(uminus!(int!(2))));

    assert_eq!(expression("+2"), Ok(uplus!(int!(2))));

    assert_eq!(expression("-2 + 4"), Ok(add!(uminus!(int!(2)), int!(4))));
    assert_eq!(expression("-(2 + 4)"), Ok(uminus!(add!(int!(2), int!(4)))));
}

#[test]
fn test_conjugate() {
    assert_eq!(expression("2`"), Ok(conj!(int!(2))));
    assert_eq!(expression("2` * 3"), Ok(multiply!(conj!(int!(2)), int!(3))));
    assert_eq!(expression("2 * 3`"), Ok(multiply!(int!(2), conj!(int!(3)))));
}
