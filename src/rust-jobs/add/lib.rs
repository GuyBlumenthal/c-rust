// #[cfg(feature = "feature_no_std")]
#![no_std]
pub use sea;
// pub use sea_rs_common::CAllocator;

// Code for no_std module
// Define a sea_nd function
sea::define_sea_nd!(sea_nd_arg, i32, 42);

// Entry point for the proof
#[no_mangle]
pub extern "C" fn entrypt() {
    test_test1();
    test_test2();
}

#[no_mangle]
fn test_test1() {
    let v = sea_nd_arg();
    sea::assume(v >= 1);
    let res = add(v, 7);

    if v > 0 {
        sea::sassert!(res > 7);
    } else {
        sea::sassert!(res <= 7);
    }
}

#[no_mangle]
fn test_test2() {
    let v = sea_nd_arg();
    sea::assume(v >= 1);
    let res = add(v, 8);

    if v > 0 {
        sea::sassert!(res > 8);
    } else {
        sea::sassert!(res <= 8);
    }
}

// Function being verified
#[no_mangle]
fn add(x: i32, y: i32) -> i32 {
    x + y
}
