#![feature(never_type, exhaustive_patterns, proc_macro_hygiene)]
use powerset_enum::*;

#[derive(Debug, PartialEq)]
struct Exception1;
#[derive(Debug, PartialEq)]
struct Exception2;
#[derive(Debug, PartialEq)]
struct Exception3;
#[derive(Debug, PartialEq)]
struct Exception4;

#[powerset_enum]
#[derive(Debug, PartialEq)]
enum Error {
    Exception1(Exception1),
    Exception2(Exception2),
    Exception3(Exception3),
    Exception4(Exception4),
}

#[test]
fn test_simple_upcast() {
    fn foo() -> Result<(), Error![Exception3]> {
        Ok(Err(Exception3)?)
    }

    fn bar() -> Result<(), Error![Exception1, Exception4, Exception3]> {
        Ok(foo().map_err(Error::upcast)?)
    }

    assert!(bar() == Err(Exception3.into()));
}

fn cause_error(n: usize) -> Result<usize, Error![Exception1, Exception2, Exception3, Exception4]> {
    match n {
        1 => Err(Exception1)?,
        2 => Err(Exception2)?,
        3 => Err(Exception3)?,
        4 => Err(Exception4)?,
        _ => Ok(n),
    }
}

#[test]
fn test_extract_variant() {

    assert!(cause_error(2).extract::<Exception2>() == Err(Exception2));
    assert!(cause_error(4).extract::<Exception2>() == Ok(Err(Error::Exception4(Exception4))));
    assert!(cause_error(6).extract::<Exception2>() == Ok(Ok(6)));
}
