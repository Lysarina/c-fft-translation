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
use crate::mcomplex::complex_t;
use crate::mcomplex::conv_from_polar;
use crate::mcomplex::add;
use crate::mcomplex::multiply;

const PI: f64 = 3.1415926535897932384626434;

unsafe extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    // fn free(_: *mut libc::c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DFT_naive_external(
    mut x: *mut complex,
    mut N: i32,
) -> *mut complex {
    let mut X: *mut complex = malloc(
            (::core::mem::size_of::<complex_t>() as u64)
                .wrapping_mul(N as u64),
        ) as *mut complex;
    for k in 0..N {
        for n in 0..N {
            // X[k as usize] = add(X[k as usize], multiply(x[n as usize], conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)))
            *X.offset(
                            k as isize,
                        ) = add(
                        *X.offset(k as isize),
                        multiply(
                            *x.offset(n as isize),
                            conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)
                        ),
                    );
        }
    }
    return X;
}

//pub type complex = complex_t;
#[unsafe(no_mangle)]
pub fn DFT_naive(
    mut x: &Vec<complex>,
    mut N: i32,
) -> Vec<complex> {
    let mut X = vec![complex { re: 0., im: 0.}; N as usize];
    // let mut X: *mut complex = malloc(
    //     (::core::mem::size_of::<complex_t>() as u64)
    //         .wrapping_mul(N as u64),
    // ) as *mut complex;
    for k in 0..N {
        for n in 0..N {
            X[k as usize] = add(X[k as usize], multiply(x[n as usize], conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)))
        }
    }
    // while k < N {
    //     (*X.offset(k as isize)).re = 0.0;
    //     (*X.offset(k as isize)).im = 0.0;
    //     n = 0;
    //     while n < N {
    //         *X
    //             .offset(
    //                 k as isize,
    //             ) = add(
    //             *X.offset(k as isize),
    //             multiply(
    //                 *x.offset(n as isize),
    //                 conv_from_polar(1., -2. * PI * n as f64 * k as f64/N as f64)
    //             ),
    //         );
    //         n += 1;
    //     }
    //     k += 1;
    // }
    return X;
    // return X;
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

    // let mut columns: *mut *mut complex = malloc(
    //     (::core::mem::size_of::<*mut complex_t>() as u64)
    //         .wrapping_mul(N1 as u64),
    // ) as *mut *mut complex;
    // while k1 < N1 {
    //     let ref mut fresh0 = *columns.offset(k1 as isize);
    //     *fresh0 = malloc(
    //         (::core::mem::size_of::<complex_t>() as u64)
    //             .wrapping_mul(N2 as u64),
    //     ) as *mut complex;
    //     k1 += 1;
    //     k1;
    // }
    // let mut rows: *mut *mut complex = malloc(
    //     (::core::mem::size_of::<*mut complex_t>() as u64)
    //         .wrapping_mul(N2 as u64),
    // ) as *mut *mut complex;
    // k2 = 0 as i32;
    // while k2 < N2 {
    //     let ref mut fresh1 = *rows.offset(k2 as isize);
    //     *fresh1 = malloc(
    //         (::core::mem::size_of::<complex_t>() as u64)
    //             .wrapping_mul(N1 as u64),
    //     ) as *mut complex;
    //     k2 += 1;
    //     k2;
    // }

    for z in 0..30 { // here we would actually need to do a runtime check for length
        k1 = z % N1;
        k2 = z % N2;
        columns[k1 as usize][k2 as usize] = *input.offset(z as isize);
    }
    // while z < 30 as i32 {
    //     k1 = z % N1;
    //     k2 = z % N2;
    //     *(*columns.offset(k1 as isize)).offset(k2 as isize) = *input.offset(z as isize);
    //     z += 1;
    //     z;
    // }
    for k1 in 0..N1 {
        columns[k1 as usize] = DFT_naive(&columns[k1 as usize], N2);
    }
    // while k1 < N1 {
    //     let ref mut fresh2 = *columns.offset(k1 as isize);
    //     *fresh2 = DFT_naive(*columns.offset(k1 as isize), N2);
    //     k1 += 1;
    //     k1;
    // }
    for k1 in 0..N1 {
        for k2 in 0..N2 {
            rows[k2 as usize][k1 as usize] = columns[k1 as usize][k2 as usize];
        }
    }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     k2 = 0 as i32;
    //     while k2 < N2 {
    //         *(*rows.offset(k2 as isize))
    //             .offset(
    //                 k1 as isize,
    //             ) = *(*columns.offset(k1 as isize)).offset(k2 as isize);
    //         k2 += 1;
    //         k2;
    //     }
    //     k1 += 1;
    //     k1;
    // }
    for k2 in 0..N2 {
        rows[k2 as usize] = DFT_naive(&rows[k2 as usize], N1);
    }
    // k2 = 0 as i32;
    // while k2 < N2 {
    //     let ref mut fresh3 = *rows.offset(k2 as isize);
    //     *fresh3 = DFT_naive(*rows.offset(k2 as isize), N1);
    //     k2 += 1;
    //     k2;
    // }

    // SAFE OUTPUT
    // let mut output = vec![complex { re: 0., im: 0.}; N as usize];
    // for k1 in 0..N1 {
    //     for k2 in 0..N2 {
    //         z = N1*k2 + N2*k1;
    //         output[(z%N) as usize] = rows[k2 as usize][k1 as usize];
    //     }
    // }

    // UNSAFE OUTPUT
    let mut output: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as u64)
            .wrapping_mul(N as u64),
    ) as *mut complex;
    for k1 in 0..N1 {
        for k2 in 0..N2 {
            z = N1*k2 + N2*k1;
            *output.offset((z % N) as isize) = rows[k2 as usize][k1 as usize];
        }
    }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     k2 = 0 as i32;
    //     while k2 < N2 {
    //         z = N1 * k2 + N2 * k1;
    //         *output
    //             .offset(
    //                 (z % N) as isize,
    //             ) = *(*rows.offset(k2 as isize)).offset(k1 as isize);
    //         k2 += 1;
    //         k2;
    //     }
    //     k1 += 1;
    //     k1;
    // }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     free(*columns.offset(k1 as isize) as *mut libc::c_void);
    //     k1 += 1;
    //     k1;
    // }
    // k2 = 0 as i32;
    // while k2 < N2 {
    //     free(*rows.offset(k2 as isize) as *mut libc::c_void);
    //     k2 += 1;
    //     k2;
    // }
    // free(columns as *mut libc::c_void);
    // free(rows as *mut libc::c_void);
    return output;
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

    // let mut columns: *mut *mut complex = malloc(
    //     (::core::mem::size_of::<*mut complex_t>() as u64)
    //         .wrapping_mul(N1 as u64),
    // ) as *mut *mut complex;
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     let ref mut fresh4 = *columns.offset(k1 as isize);
    //     *fresh4 = malloc(
    //         (::core::mem::size_of::<complex_t>() as u64)
    //             .wrapping_mul(N2 as u64),
    //     ) as *mut complex;
    //     k1 += 1;
    //     k1;
    // }
    // let mut rows: *mut *mut complex = malloc(
    //     (::core::mem::size_of::<*mut complex_t>() as u64)
    //         .wrapping_mul(N2 as u64),
    // ) as *mut *mut complex;
    // k2 = 0 as i32;
    // while k2 < N2 {
    //     let ref mut fresh5 = *rows.offset(k2 as isize);
    //     *fresh5 = malloc(
    //         (::core::mem::size_of::<complex_t>() as u64)
    //             .wrapping_mul(N1 as u64),
    //     ) as *mut complex;
    //     k2 += 1;
    //     k2;
    // }

    for k1 in 0..N1 {
        for k2 in 0..N2 {
            columns[k1 as usize][k2 as usize] = *input.offset((N1 * k2 + k1) as isize)
        }
    }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     k2 = 0 as i32;
    //     while k2 < N2 {
    //         *(*columns.offset(k1 as isize))
    //             .offset(k2 as isize) = *input.offset((N1 * k2 + k1) as isize);
    //         k2 += 1;
    //         k2;
    //     }
    //     k1 += 1;
    //     k1;
    // }

    for k1 in 0..N1 {
        columns[k1 as usize] = DFT_naive(&columns[k1 as usize], N2);
    }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     let ref mut fresh6 = *columns.offset(k1 as isize);
    //     *fresh6 = DFT_naive(*columns.offset(k1 as isize), N2);
    //     k1 += 1;
    //     k1;
    // }

    for k1 in 0..N1 {
        for k2 in 0..N2 {
            rows[k2 as usize][k1 as usize] = multiply(
                conv_from_polar(1., -2. * PI * k1 as f64 * k2 as f64/N as f64),
                columns[k1 as usize][k2 as usize]
            );
        }
    }

    // k1 = 0 as i32;
    // while k1 < N1 {
    //     k2 = 0 as i32;
    //     while k2 < N2 {
    //         *(*rows.offset(k2 as isize))
    //             .offset(
    //                 k1 as isize,
    //             ) = multiply(
    //             conv_from_polar(
    //                 1 as i32 as f64,
    //                 -2.0f64 * 3.1415926535897932384626434f64 * k1 as f64
    //                     * k2 as f64 / N as f64,
    //             ),
    //             *(*columns.offset(k1 as isize)).offset(k2 as isize),
    //         );
    //         k2 += 1;
    //         k2;
    //     }
    //     k1 += 1;
    //     k1;
    // }

    for k2 in 0..N2 {
        rows[k2 as usize] = DFT_naive(&rows[k2 as usize], N1);
    }

    // k2 = 0 as i32;
    // while k2 < N2 {
    //     let ref mut fresh7 = *rows.offset(k2 as isize);
    //     *fresh7 = DFT_naive(*rows.offset(k2 as isize), N1);
    //     k2 += 1;
    //     k2;
    // }

    let mut output: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as u64)
            .wrapping_mul(N as u64),
    ) as *mut complex;
    for k1 in 0..N1 {
        for k2 in 0..N2 {
            *output.offset((N2 * k1 + k2) as isize) = rows[k2 as usize][k1 as usize];
        }
    }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     k2 = 0 as i32;
    //     while k2 < N2 {
    //         *output
    //             .offset(
    //                 (N2 * k1 + k2) as isize,
    //             ) = *(*rows.offset(k2 as isize)).offset(k1 as isize);
    //         k2 += 1;
    //         k2;
    //     }
    //     k1 += 1;
    //     k1;
    // }
    // k1 = 0 as i32;
    // while k1 < N1 {
    //     free(*columns.offset(k1 as isize) as *mut libc::c_void);
    //     k1 += 1;
    //     k1;
    // }
    // k2 = 0 as i32;
    // while k2 < N2 {
    //     free(*rows.offset(k2 as isize) as *mut libc::c_void);
    //     k2 += 1;
    //     k2;
    // }
    // free(columns as *mut libc::c_void);
    // free(rows as *mut libc::c_void);
    return output;
}
