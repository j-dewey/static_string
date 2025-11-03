use crate::static_str::StaticString;

mod global;
mod pool;
mod static_str;

const HELLO: StaticString = StaticString::from_static_str("Hello");

fn main() {
    let string = " world!".to_owned();
    let world = StaticString::from_str(&string[..]);
    println!("{}{}", HELLO, world);
}
