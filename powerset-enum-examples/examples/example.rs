use powerset_enum::*;

#[powerset_enum]
pub enum Error {
    A,
    B,
}

impl Error {
    fn foo() -> &'static str {
        "hello"
    }
}

impl WithVariant<Error> for Error {
    type With = Self;
}

fn main() {
    println!("here be example {}", Error::foo());
}
