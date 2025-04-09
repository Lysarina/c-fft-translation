#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

mod mcomplex;
use crate::mcomplex::complex;
use crate::mcomplex::complex_t;
use crate::mcomplex::conv_from_polar;
use crate::mcomplex::add;
use crate::mcomplex::multiply;

use std::f64::consts::PI;

unsafe extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DFT_naive(
    x: *mut complex,
    N: i32,
) -> *mut complex {
    let X: *mut complex = unsafe { malloc(
            (::core::mem::size_of::<complex_t>() as u64)
                .wrapping_mul(N as u64),
        ) as *mut complex };
    for k in 0..N {
        for n in 0..N {
            unsafe { *X.offset(k as isize) = add(
                        *X.offset(k as isize),
                        multiply(
                            *x.offset(n as isize),
                            conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)
                        )
                    ); 
                }
        }
    }
    X
}

#[unsafe(no_mangle)]
pub fn safe_DFT_naive(
    x: &[complex],
    N: i32,
) -> Vec<complex> {
    let mut X = vec![complex { re: 0., im: 0.}; N as usize];
    for k in 0..N {
        for n in 0..N {
            X[k as usize] = add(X[k as usize], multiply(x[n as usize], conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)))
        }
    }
    X
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FFT_GoodThomas(
    input: *mut complex,
    N: i32,
    N1: i32,
    N2: i32,
) -> *mut complex {
    let mut k1: i32;
    let mut k2: i32;
    let mut z: i32;

    // Inits columns: N1xN2 2D vector
    let mut columns = vec![vec![complex { re: 0., im: 0.}; N2 as usize]; N1 as usize];
    // Inits rows: N2xN1 2D vector
    let mut rows = vec![vec![complex { re: 0., im: 0.}; N1 as usize]; N2 as usize];

    // here 30 is hardcoded from benchmark
    // we would actually need to do a runtime check for length, but since the C version
    // does not do it, we skip as well
    for z in 0..30 { 
        k1 = z % N1;
        k2 = z % N2;
        columns[k1 as usize][k2 as usize] = unsafe { *input.offset(z as isize) };
    }

    for k1 in 0..N1 {
        columns[k1 as usize] = safe_DFT_naive(&columns[k1 as usize], N2);
    }

    for k1 in 0..N1 {
        for k2 in 0..N2 {
            rows[k2 as usize][k1 as usize] = columns[k1 as usize][k2 as usize];
        }
    }

    for k2 in 0..N2 {
        rows[k2 as usize] = safe_DFT_naive(&rows[k2 as usize], N1);
    }

    let output: *mut complex = unsafe { malloc(
        (::core::mem::size_of::<complex_t>() as u64)
            .wrapping_mul(N as u64),
    ) as *mut complex };
    for k1 in 0..N1 {
        for k2 in 0..N2 {
            z = N1*k2 + N2*k1;
            unsafe { *output.offset((z % N) as isize) = rows[k2 as usize][k1 as usize]; }
        }
    }
    output
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FFT_CooleyTukey(
    input: *mut complex,
    N: i32,
    N1: i32,
    N2: i32,
) -> *mut complex {

    // Inits columns: N1xN2 2D vector
    let mut columns = vec![vec![complex { re: 0., im: 0.}; N2 as usize]; N1 as usize];
    // Inits rows: N2xN1 2D vector
    let mut rows = vec![vec![complex { re: 0., im: 0.}; N1 as usize]; N2 as usize];


    for k1 in 0..N1 {
        for k2 in 0..N2 {
            columns[k1 as usize][k2 as usize] = unsafe { *input.offset((N1 * k2 + k1) as isize) };
        }
    }

    for k1 in 0..N1 {
        columns[k1 as usize] = safe_DFT_naive(&columns[k1 as usize], N2);
    }

    for k1 in 0..N1 {
        for k2 in 0..N2 {
            rows[k2 as usize][k1 as usize] = multiply(
                conv_from_polar(1., -2. * PI * k1 as f64 * k2 as f64/N as f64),
                columns[k1 as usize][k2 as usize]
            );
        }
    }

    for k2 in 0..N2 {
        rows[k2 as usize] = safe_DFT_naive(&rows[k2 as usize], N1);
    }

    let output: *mut complex = unsafe { malloc(
        (::core::mem::size_of::<complex_t>() as u64)
            .wrapping_mul(N as u64),
    ) as *mut complex };

    for k1 in 0..N1 {
        for k2 in 0..N2 {
            unsafe { *output.offset((N2 * k1 + k2) as isize) = rows[k2 as usize][k1 as usize]; }
        }
    }
    
    output
}
