// SPDX-License-Identifier: BSD-2-Clauses

use std::os::raw::c_char;

// TODO replace with the variadic when it is stable
#[no_mangle]
extern "C" fn checkasm_fail_func(_fmt: *const c_char) -> i32 {
    eprintln!("checkasm failed");
    0
}

/// declare a function checker with the correct arguments
///
/// pass the function to check as first argument followed by the function arguments.
#[macro_export]
macro_rules! declare_fn {
    ($fun:ident $($name:ident : $ty:ty),+) => {
        cfg_if::cfg_if! {
            if #[cfg(target_arch="x86_64")] {
                fn $fun(func: *mut std::ffi::c_void, $($name: $ty),+) {
                    const CLOB: u64 = 0xdeadbeefdeadbeef;

                    extern "C" {
                        fn checkasm_checked_call(func: *mut c_void, ...);
                        fn checkasm_stack_clobber(clobber: u64, ...);
                    }

                    unsafe {
                        checkasm_stack_clobber(CLOB, CLOB, CLOB, CLOB, CLOB, CLOB, CLOB,
                                               CLOB, CLOB, CLOB, CLOB, CLOB, CLOB, CLOB,
                                               CLOB, CLOB, CLOB, CLOB, CLOB, CLOB, CLOB);
                        #[cfg(target_os="windows")] {
                            checkasm_checked_call(func, 0, 0, 0, 0, 0, $($name),+,
                                                  11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0);
                        }
                        #[cfg(not(target_os="windows"))] {
                            checkasm_checked_call(func, 0, 0, 0, 0, 0, $($name),+,
                                                  9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0);
                        }

                    }
                }
            } else if #[cfg(target_arch="aarch64")] {
                fn $fun(func: *mut std::ffi::c_void, $($name: $ty),+) {
                    const CLOB: u64 = 0xdeadbeefdeadbeef;

                    extern "C" {
                        fn checkasm_checked_call(func: *mut c_void, ...);
                        fn checkasm_stack_clobber(clobber: u64, ...);
                    }

                    unsafe {
                        checkasm_stack_clobber(CLOB, CLOB, CLOB, CLOB, CLOB, CLOB,
                                               CLOB, CLOB, CLOB, CLOB, CLOB, CLOB,
                                               CLOB, CLOB, CLOB, CLOB, CLOB, CLOB,
                                               CLOB, CLOB, CLOB, CLOB, CLOB);
                        checkasm_checked_call(func, 0, 0, 0, 0, 0, $($name),+,
                                              7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0);
                    }
                }
            } else {
                fn $fun(func: *mut c_void, $($name: $ty),+) {
                    unsafe {
                        let f: fn($($ty),+) = std::mem::transmute(func);
                        f($($name),+);
                    }
                }
            }
        }
    }
}
