#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod mcomplex;
use crate::mcomplex::complex;
use crate::mcomplex::conv_from_polar;
use crate::mcomplex::add;
use crate::mcomplex::multiply;

const PI: f64 = 3.1415926535897932384626434;

unsafe extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DFT_naive(
    mut x: *mut complex,
    mut N: i32,
) -> *mut complex {
    let mut X = vec![complex { re: 0., im: 0.}; N as usize];
    for k in 0..N {
        for n in 0..N {
            X[k as usize] = add(X[k as usize], multiply(
                unsafe { *x.offset(n as isize) },
                conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64))
            )
        }
    }

    let mut boxed_slice = X.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();

    std::mem::forget(boxed_slice);
    return ptr;
}

#[unsafe(no_mangle)]
pub fn safe_DFT_naive(
    mut x: &Vec<complex>,
    mut N: i32,
) -> Vec<complex> {
    let mut X = vec![complex { re: 0., im: 0.}; N as usize];
    for k in 0..N {
        for n in 0..N {
            X[k as usize] = add(X[k as usize], multiply(x[n as usize], conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)))
        }
    }
    return X;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FFT_GoodThomas(
    mut input: *mut complex,
    mut N: i32,
    mut N1: i32,
    mut N2: i32,
) -> *mut complex {
    let mut k1: i32 = 0;
    let mut k2: i32 = 0;
    let mut z: i32 = 0;

    // Inits columns: N1xN2 2D vector
    let mut columns = vec![vec![complex { re: 0., im: 0.}; N2 as usize]; N1 as usize];
    // Inits rows: N2xN1 2D vector
    let mut rows = vec![vec![complex { re: 0., im: 0.}; N1 as usize]; N2 as usize];

    for z in 0..30 { // here we would actually need to do a runtime check for length
        k1 = z % N1;
        k2 = z % N2;
        columns[k1 as usize][k2 as usize] = unsafe {*input.offset(z as isize)};
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

    let mut output = vec![complex { re: 0., im: 0.}; N as usize];
    for k1 in 0..N1 {
        for k2 in 0..N2 {
            z = N1*k2 + N2*k1;
            output[(z%N) as usize] = rows[k2 as usize][k1 as usize];
        }
    }

    let mut boxed_slice = output.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();

    std::mem::forget(boxed_slice);
    return ptr;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FFT_CooleyTukey(
    mut input: *mut complex,
    mut N: i32,
    mut N1: i32,
    mut N2: i32,
) -> *mut complex {

    // Inits columns: N1xN2 2D vector
    let mut columns = vec![vec![complex { re: 0., im: 0.}; N2 as usize]; N1 as usize];
    // Inits rows: N2xN1 2D vector
    let mut rows = vec![vec![complex { re: 0., im: 0.}; N1 as usize]; N2 as usize];

    for k1 in 0..N1 {
        for k2 in 0..N2 {
            columns[k1 as usize][k2 as usize] = unsafe{*input.offset((N1 * k2 + k1) as isize)}
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

    let mut output = vec![complex { re: 0., im: 0.}; N as usize];
    for k1 in 0..N1 {
        for k2 in 0..N2 {
            output[(N2 * k1 + k2) as usize] = rows[k2 as usize][k1 as usize];
        }
    }
    let mut boxed_slice = output.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();

    std::mem::forget(boxed_slice);
    return ptr;
}
