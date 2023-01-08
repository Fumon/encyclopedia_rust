#![allow(unused_imports)]
use std::{num::{IntErrorKind, ParseIntError}};

#[test]
fn same_error_type() {
    let r1 = "16".parse::<u16>();
    let r2 = "86".parse::<u16>();
    let r3 = "-12".parse::<u16>();

    let res1 = r1.and_then(|mi1| r2.map(|mi2| mi1 + mi2));
    assert_eq!(Ok(102), res1);

    let res2 = res1.and_then(|mres1| r3.map(|mi3| mi3 + mres1));

    match res2 {
        Ok(_) => panic!("Should not parse"),
        Err(e) => assert_eq!(e.kind(), &IntErrorKind::InvalidDigit),
    }
}

#[test]
fn different_error_types() {
    let int_r = "19".parse::<u16>();
    let ok1: Result<u16, &'static str> = Ok(7);
    let err1: Result<u16, bool> = Err(false);

    // Can't work
    // let end = intR
    //     .and_then(|intRint| ok1.map(|ok1int| ok1int + intRint))
    //     .and_then(|rplusok| err1.map(|err1int| rplusok + err1int));

    // Introduce an error type

    enum CommonError {
        ParseIntError(ParseIntError),
        SStringError(&'static str),
        BoolError(bool),
    }

    impl From<ParseIntError> for CommonError {
        fn from(value: ParseIntError) -> Self {
            Self::ParseIntError(value)
        }
    }

    impl From<&'static str> for CommonError {
        fn from(value: &'static str) -> Self {
            Self::SStringError(value)
        }
    }

    impl From<bool> for CommonError {
        fn from(value: bool) -> Self {
            Self::BoolError(value)
        }
    }

    let res = || -> Result<u16, CommonError> { Ok(int_r? + ok1? + err1?) }();

    match res {
        Err(CommonError::BoolError(f)) if f == false => assert_eq!(f, false),
        Ok(_) | Err(_) => panic!("Should not succeed or error with anything else"),
    }
}
