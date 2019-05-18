// #![feature(never_type, proc_macro_hygiene)]
// use powerset_enum::*;

// #[derive(Debug)]
// pub struct Exception1;
// #[derive(Debug)]
// pub struct Exception2;
// #[derive(Debug)]
// pub struct Exception3;

// #[powerset_enum]
// #[derive(Debug)]
// pub enum Error {
    // Exception1(Exception1),
    // Exception2(Exception2),
    // Exception3(Exception3),
// }

// fn f1(x: u32) -> Result<u32, Error!(Exception1, Exception2, Exception3, )> {
    // match x {
        // 1 => Err(Error::Exception1(Exception1)),
        // 2 => Err(Error::Exception2(Exception2)),
        // 3 => Err(Error::Exception3(Exception3)),
        // x => Ok(x),
    // }
// }

fn main() {
    // for i in 0..=4 {
        // println!("i={}\tf1(i)={:?}", i, f1(i));
    // }
    // println!("{:?}", std::env::current_dir());
}
