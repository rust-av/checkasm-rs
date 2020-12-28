# checkasm helper for rust

It is a port of the [dav1d](https://code.videolan.org/videolan/dav1d) checkasm harness to rust.

## Usage

``` rust

#[cfg(test)]
mod test {
    use checkasm::declare_fn;
    use std::ffi::c_void;

    extern fn variant_avx2(a: *mut u8, len: usize);
    extern fn variant_avx512(a: *mut u8, len: usize);

    declare_fn { check_variant(a: *mut u8, len: usize) };

    #[test]
    fn variant() {
        let mut buf = vec![0u8; 128];

        check_variant(variant_avx512 as *mut c_void, buf.as_mut_ptr(), buf.len());
    }
}
```

## Status

- [x] builds
- [x] reports errors on stderr
- [ ] panics correctly

