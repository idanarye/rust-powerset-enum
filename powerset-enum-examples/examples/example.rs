#![feature(never_type, proc_macro_hygiene)]
use powerset_enum::*;

#[derive(Debug)]
pub struct Exception1;
#[derive(Debug)]
pub struct Exception2;
#[derive(Debug)]
pub struct Exception3;

#[powerset_enum]
pub enum Error {
    Exception1(Exception1),
    Exception2(Exception2),
    Exception3(Exception3),
}

// impl Error {
    // fn foo() -> &'static str {
        // "hello"
    // }
// }

// impl WithVariant<Error> for Error {
    // type With = Self;
// }

fn main() {
    // println!("here be example {}", Error::foo());
    println!("le example");
}
