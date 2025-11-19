//
// Tests to ensure that PooledStrings constructed from String and non static &str
// behave as expected
//

use static_string::{PooledString, is_pool_init};

const DYNAMIC_STRING: &'static str = "dynamic string";

#[test]
fn alloc_dynamic() {
    let initial_string = String::from(DYNAMIC_STRING);
    let pooled = PooledString::from_str(&initial_string);
    assert_eq!(pooled, *DYNAMIC_STRING);
    assert!(is_pool_init());
}

#[test]
fn is_moveable() {
    fn make_pooled() -> PooledString {
        let initial = String::from(DYNAMIC_STRING);
        PooledString::from_str(&initial)
    }

    let pooled = make_pooled();
    assert_eq!(pooled, *DYNAMIC_STRING);
    assert!(is_pool_init());
}
