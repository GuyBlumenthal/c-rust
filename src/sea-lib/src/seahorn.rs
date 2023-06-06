use crate::bindings::*;

#[no_mangle]
pub fn verifier_error() {
    unsafe {
        __VERIFIER_error();
    }
}

#[no_mangle]
pub fn assume(v: bool) {
    unsafe {
        __VERIFIER_assume(v.into());
    }
}

#[no_mangle]
pub fn nd_i32() -> i32 {
    unsafe { sea_nd_i32() }
}

#[no_mangle]
pub fn nd_bool() -> bool {
    unsafe { sea_nd_bool() }
}

#[macro_export]
macro_rules! sassert {
    ($cond:expr) => {{
        if !$cond {
            sea::verifier_error();
        }
    }};
}

/// Defines `sea_nd` function that returns nd value
///
/// The implementation generates a sea_nd function that
/// attempts to confuse Rust optimizer from understanding what
/// is the return of the function.
///
/// The first argument specifies the defined function name.
/// It must start with the prefix `sea_nd`.
///
/// The second argument is the type for nondet values. The type
/// is assumed to implement the Default trait
///
/// The third argument is a value to return in the implementation.
/// It must be any value that is different from default value for the type.
///
/// # Example
///
/// define_sea_nd!(sea_nd_foo, i32, 42);
///
#[macro_export]
macro_rules! define_sea_nd {
    ($name:ident,$typ:ty,$val:expr) => {
        #[no_mangle]
        #[inline(never)]
        pub extern "C" fn $name() -> $typ {
            if sea::nd_bool() {
                <$typ>::default()
            } else {
                $val
            }
        }
    };
}
