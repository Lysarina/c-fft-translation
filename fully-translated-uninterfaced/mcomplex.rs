// #![allow(
//     dead_code,
//     mutable_transmutes,
//     non_camel_case_types,
//     non_snake_case,
//     non_upper_case_globals,
//     unused_assignments,
//     unused_mut
// )]

#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex_t {
    pub re: f64,
    pub im: f64,
}

pub type complex = complex_t;
#[unsafe(no_mangle)]
pub fn conv_from_polar(
    mut r: f64,
    mut radians: f64,
) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = r * radians.cos();
    result.im = r * radians.sin();
    return result;
}

#[unsafe(no_mangle)]
pub fn add(mut left: complex, mut right: complex) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = left.re + right.re;
    result.im = left.im + right.im;
    return result;
}

#[unsafe(no_mangle)]
pub fn multiply(mut left: complex, mut right: complex) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = left.re * right.re - left.im * right.im;
    result.im = left.re * right.im + left.im * right.re;
    return result;
}
