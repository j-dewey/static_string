#![feature(lazy_get)]
use crate::pstring::PooledString;

mod global;
mod pool;
mod pstring;

const HELLO: PooledString = PooledString::from_static_str("Hello");

fn main() {
    let string = " world!".to_owned();
    let world = PooledString::from_str(&string[..]);
    println!("{}{}", HELLO, world);
}
