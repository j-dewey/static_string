//
// Tests to ensure that PoolStrings made from &static str behave as expected
//

use static_string::{PooledString, is_pool_init};

const STATIC_STR: &'static str = "static string";

#[test]
fn static_allocate() {
    let string = PooledString::from_static_str(STATIC_STR);
    assert_eq!(string, *STATIC_STR);
    assert!(!is_pool_init());
}

#[test]
fn static_no_dealloc() {
    let refr: &'static str = "static string";
    {
        let _ = PooledString::from_static_str(refr);
    }
    assert_eq!(*refr, *STATIC_STR);
}
