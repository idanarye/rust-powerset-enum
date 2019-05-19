#![feature(never_type, exhaustive_patterns, proc_macro_hygiene)]
use powerset_enum::*;

#[derive(Debug, PartialEq)]
pub struct Exception1;
#[derive(Debug, PartialEq)]
pub struct Exception2;
#[derive(Debug, PartialEq)]
pub struct Exception3;

#[powerset_enum]
#[derive(Debug, PartialEq)]
pub enum Error {
    Exception1(Exception1),
    Exception2(Exception2),
    Exception3(Exception3),
}

fn foo(x: usize) -> Result<usize, Error![Exception1, Exception2]> {
    Ok(match x {
        1 => Err(Exception1)?,
        2 => Err(Exception2)?,
        x => x,
    })
}

fn bar(x: usize) -> Result<usize, Error![Exception1, Exception3]> {
    if x == 3 {
        Err(Exception3)?;
    }
    foo(x)
        // Specifically handle `Exception2`:
        .extract::<Exception2>().unwrap_or(Ok(2))
        // Convert `Result<usize, Error![Exception1]>` to `Result<usize, Error![Exception1, Exception3]>`:
        .map_err(Error::upcast)
}

fn main() {
    let x = 1;

    match foo(x) {
        Ok(n) => println!("OK - got {}", n),
        Err(Error::Exception1(_)) => println!("Got exception 1"),
        Err(Error::Exception2(_)) => println!("Got exception 2"),
        // No Exception3 match arm needed - `foo` cannot return it
    }

    match bar(x) {
        Ok(n) => println!("OK - got {}", n),
        Err(Error::Exception1(_)) => println!("Got exception 1"),
        // No Exception2 match arm needed - `bar` cannot return it
        Err(Error::Exception3(_)) => println!("Got exception 3"),
    }
}
