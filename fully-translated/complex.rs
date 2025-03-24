#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex_t {
    pub re: libc::c_double,
    pub im: libc::c_double,
}
pub type complex = complex_t;
#[no_mangle]
pub unsafe extern "C" fn conv_from_polar(
    mut r: libc::c_double,
    mut radians: libc::c_double,
) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = r * cos(radians);
    result.im = r * sin(radians);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn add(mut left: complex, mut right: complex) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = left.re + right.re;
    result.im = left.im + right.im;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn multiply(mut left: complex, mut right: complex) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = left.re * right.re - left.im * right.im;
    result.im = left.re * right.im + left.im * right.re;
    return result;
}
