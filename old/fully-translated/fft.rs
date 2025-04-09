mod complex;
use crate::complex;

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
unsafe extern "C" {
    fn conv_from_polar(r: libc::c_double, radians: libc::c_double) -> complex;
    fn add(left: complex, right: complex) -> complex;
    fn multiply(left: complex, right: complex) -> complex;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex_t {
    pub re: libc::c_double,
    pub im: libc::c_double,
}
pub type complex = complex_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DFT_naive(
    mut x: *mut complex,
    mut N: libc::c_int,
) -> *mut complex {
    let mut X: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as libc::c_ulong)
            .wrapping_mul(N as libc::c_ulong),
    ) as *mut complex;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < N {
        (*X.offset(k as isize)).re = 0.0f64;
        (*X.offset(k as isize)).im = 0.0f64;
        n = 0 as libc::c_int;
        while n < N {
            *X
                .offset(
                    k as isize,
                ) = add(
                *X.offset(k as isize),
                multiply(
                    *x.offset(n as isize),
                    conv_from_polar(
                        1 as libc::c_int as libc::c_double,
                        -(2 as libc::c_int) as libc::c_double
                            * 3.1415926535897932384626434f64 * n as libc::c_double
                            * k as libc::c_double / N as libc::c_double,
                    ),
                ),
            );
            n += 1;
            n;
        }
        k += 1;
        k;
    }
    return X;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FFT_GoodThomas(
    mut input: *mut complex,
    mut N: libc::c_int,
    mut N1: libc::c_int,
    mut N2: libc::c_int,
) -> *mut complex {
    let mut k1: libc::c_int = 0;
    let mut k2: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut columns: *mut *mut complex = malloc(
        (::core::mem::size_of::<*mut complex_t>() as libc::c_ulong)
            .wrapping_mul(N1 as libc::c_ulong),
    ) as *mut *mut complex;
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        let ref mut fresh0 = *columns.offset(k1 as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<complex_t>() as libc::c_ulong)
                .wrapping_mul(N2 as libc::c_ulong),
        ) as *mut complex;
        k1 += 1;
        k1;
    }
    let mut rows: *mut *mut complex = malloc(
        (::core::mem::size_of::<*mut complex_t>() as libc::c_ulong)
            .wrapping_mul(N2 as libc::c_ulong),
    ) as *mut *mut complex;
    k2 = 0 as libc::c_int;
    while k2 < N2 {
        let ref mut fresh1 = *rows.offset(k2 as isize);
        *fresh1 = malloc(
            (::core::mem::size_of::<complex_t>() as libc::c_ulong)
                .wrapping_mul(N1 as libc::c_ulong),
        ) as *mut complex;
        k2 += 1;
        k2;
    }
    z = 0 as libc::c_int;
    while z < 30 as libc::c_int {
        k1 = z % N1;
        k2 = z % N2;
        *(*columns.offset(k1 as isize)).offset(k2 as isize) = *input.offset(z as isize);
        z += 1;
        z;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        let ref mut fresh2 = *columns.offset(k1 as isize);
        *fresh2 = DFT_naive(*columns.offset(k1 as isize), N2);
        k1 += 1;
        k1;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        k2 = 0 as libc::c_int;
        while k2 < N2 {
            *(*rows.offset(k2 as isize))
                .offset(
                    k1 as isize,
                ) = *(*columns.offset(k1 as isize)).offset(k2 as isize);
            k2 += 1;
            k2;
        }
        k1 += 1;
        k1;
    }
    k2 = 0 as libc::c_int;
    while k2 < N2 {
        let ref mut fresh3 = *rows.offset(k2 as isize);
        *fresh3 = DFT_naive(*rows.offset(k2 as isize), N1);
        k2 += 1;
        k2;
    }
    let mut output: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as libc::c_ulong)
            .wrapping_mul(N as libc::c_ulong),
    ) as *mut complex;
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        k2 = 0 as libc::c_int;
        while k2 < N2 {
            z = N1 * k2 + N2 * k1;
            *output
                .offset(
                    (z % N) as isize,
                ) = *(*rows.offset(k2 as isize)).offset(k1 as isize);
            k2 += 1;
            k2;
        }
        k1 += 1;
        k1;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        free(*columns.offset(k1 as isize) as *mut libc::c_void);
        k1 += 1;
        k1;
    }
    k2 = 0 as libc::c_int;
    while k2 < N2 {
        free(*rows.offset(k2 as isize) as *mut libc::c_void);
        k2 += 1;
        k2;
    }
    free(columns as *mut libc::c_void);
    free(rows as *mut libc::c_void);
    return output;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FFT_CooleyTukey(
    mut input: *mut complex,
    mut N: libc::c_int,
    mut N1: libc::c_int,
    mut N2: libc::c_int,
) -> *mut complex {
    let mut k1: libc::c_int = 0;
    let mut k2: libc::c_int = 0;
    let mut columns: *mut *mut complex = malloc(
        (::core::mem::size_of::<*mut complex_t>() as libc::c_ulong)
            .wrapping_mul(N1 as libc::c_ulong),
    ) as *mut *mut complex;
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        let ref mut fresh4 = *columns.offset(k1 as isize);
        *fresh4 = malloc(
            (::core::mem::size_of::<complex_t>() as libc::c_ulong)
                .wrapping_mul(N2 as libc::c_ulong),
        ) as *mut complex;
        k1 += 1;
        k1;
    }
    let mut rows: *mut *mut complex = malloc(
        (::core::mem::size_of::<*mut complex_t>() as libc::c_ulong)
            .wrapping_mul(N2 as libc::c_ulong),
    ) as *mut *mut complex;
    k2 = 0 as libc::c_int;
    while k2 < N2 {
        let ref mut fresh5 = *rows.offset(k2 as isize);
        *fresh5 = malloc(
            (::core::mem::size_of::<complex_t>() as libc::c_ulong)
                .wrapping_mul(N1 as libc::c_ulong),
        ) as *mut complex;
        k2 += 1;
        k2;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        k2 = 0 as libc::c_int;
        while k2 < N2 {
            *(*columns.offset(k1 as isize))
                .offset(k2 as isize) = *input.offset((N1 * k2 + k1) as isize);
            k2 += 1;
            k2;
        }
        k1 += 1;
        k1;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        let ref mut fresh6 = *columns.offset(k1 as isize);
        *fresh6 = DFT_naive(*columns.offset(k1 as isize), N2);
        k1 += 1;
        k1;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        k2 = 0 as libc::c_int;
        while k2 < N2 {
            *(*rows.offset(k2 as isize))
                .offset(
                    k1 as isize,
                ) = multiply(
                conv_from_polar(
                    1 as libc::c_int as libc::c_double,
                    -2.0f64 * 3.1415926535897932384626434f64 * k1 as libc::c_double
                        * k2 as libc::c_double / N as libc::c_double,
                ),
                *(*columns.offset(k1 as isize)).offset(k2 as isize),
            );
            k2 += 1;
            k2;
        }
        k1 += 1;
        k1;
    }
    k2 = 0 as libc::c_int;
    while k2 < N2 {
        let ref mut fresh7 = *rows.offset(k2 as isize);
        *fresh7 = DFT_naive(*rows.offset(k2 as isize), N1);
        k2 += 1;
        k2;
    }
    let mut output: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as libc::c_ulong)
            .wrapping_mul(N as libc::c_ulong),
    ) as *mut complex;
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        k2 = 0 as libc::c_int;
        while k2 < N2 {
            *output
                .offset(
                    (N2 * k1 + k2) as isize,
                ) = *(*rows.offset(k2 as isize)).offset(k1 as isize);
            k2 += 1;
            k2;
        }
        k1 += 1;
        k1;
    }
    k1 = 0 as libc::c_int;
    while k1 < N1 {
        free(*columns.offset(k1 as isize) as *mut libc::c_void);
        k1 += 1;
        k1;
    }
    k2 = 0 as libc::c_int;
    while k2 < N2 {
        free(*rows.offset(k2 as isize) as *mut libc::c_void);
        k2 += 1;
        k2;
    }
    free(columns as *mut libc::c_void);
    free(rows as *mut libc::c_void);
    return output;
}
