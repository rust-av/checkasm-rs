use checkasm::declare_fn;
use std::ffi::c_void;

declare_fn! { check_one(a:i32) }
declare_fn! { check_two(a:i32, b:i32) }

#[test]
fn declare() {
    fn one(a: i32) {
        println!("Got {}", a);
    }
    fn two(a: i32, b: i32) {
        println!("Got {} {}", a, b);
    }

    check_one(one as *mut c_void, 42);

    check_two(two as *mut c_void, 21, 21);
}
