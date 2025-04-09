#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex_t {
    pub re: f64,
    pub im: f64,
}

pub type complex = complex_t;
#[unsafe(no_mangle)]
pub fn conv_from_polar(
    r: f64,
    radians: f64,
) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = r * radians.cos();
    result.im = r * radians.sin();
    result
}

#[unsafe(no_mangle)]
pub fn add(left: complex, right: complex) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = left.re + right.re;
    result.im = left.im + right.im;
    result
}

#[unsafe(no_mangle)]
pub fn multiply(left: complex, right: complex) -> complex {
    let mut result: complex = complex_t { re: 0., im: 0. };
    result.re = left.re * right.re - left.im * right.im;
    result.im = left.re * right.im + left.im * right.re;
    result
}
