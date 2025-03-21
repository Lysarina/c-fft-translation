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
    fn FFT_CooleyTukey(
        x: *mut complex,
        N: libc::c_int,
        N1: libc::c_int,
        N2: libc::c_int,
    ) -> *mut complex;
    fn FFT_GoodThomas(
        x: *mut complex,
        N: libc::c_int,
        N1: libc::c_int,
        N2: libc::c_int,
    ) -> *mut complex;
    fn DFT_naive(x: *mut complex, N: libc::c_int) -> *mut complex;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex_t {
    pub re: libc::c_double,
    pub im: libc::c_double,
}
pub type complex = complex_t;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[no_mangle]
pub unsafe extern "C" fn timeSubtract(
    mut result: *mut timeval,
    mut t2: *mut timeval,
    mut t1: *mut timeval,
) {
    let mut diff: libc::c_long = (*t2).tv_usec
        + 1000000 as libc::c_int as __time_t * (*t2).tv_sec
        - ((*t1).tv_usec + 1000000 as libc::c_int as __time_t * (*t1).tv_sec);
    (*result).tv_sec = diff / 1000000 as libc::c_int as libc::c_long;
    (*result).tv_usec = diff % 1000000 as libc::c_int as libc::c_long;
}
unsafe fn main_0() -> libc::c_int {
    let mut tvBegin: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tvEnd: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tvDiff: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut input: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as libc::c_ulong)
            .wrapping_mul(30 as libc::c_int as libc::c_ulong),
    ) as *mut complex;
    let mut result: *mut complex = 0 as *mut complex;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        (*input.offset(i as isize)).re = i as libc::c_double;
        (*input.offset(i as isize)).im = 0.0f64;
        i += 1;
        i;
    }
    gettimeofday(&mut tvBegin, 0 as *mut libc::c_void);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 10000 as libc::c_int {
        result = DFT_naive(input, 30 as libc::c_int);
        i_0 += 1;
        i_0;
    }
    gettimeofday(&mut tvEnd, 0 as *mut libc::c_void);
    timeSubtract(&mut tvDiff, &mut tvEnd, &mut tvBegin);
    printf(
        b"100000 x Naive: \t %ld.%d\n\0" as *const u8 as *const libc::c_char,
        tvDiff.tv_sec,
        tvDiff.tv_usec,
    );
    gettimeofday(&mut tvBegin, 0 as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 10000 as libc::c_int {
        result = FFT_CooleyTukey(
            input,
            30 as libc::c_int,
            6 as libc::c_int,
            5 as libc::c_int,
        );
        i_1 += 1;
        i_1;
    }
    gettimeofday(&mut tvEnd, 0 as *mut libc::c_void);
    timeSubtract(&mut tvDiff, &mut tvEnd, &mut tvBegin);
    printf(
        b"100000 x Cooley-Tukey: \t %ld.%d\n\0" as *const u8 as *const libc::c_char,
        tvDiff.tv_sec,
        tvDiff.tv_usec,
    );
    gettimeofday(&mut tvBegin, 0 as *mut libc::c_void);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 10000 as libc::c_int {
        result = FFT_GoodThomas(
            input,
            30 as libc::c_int,
            6 as libc::c_int,
            5 as libc::c_int,
        );
        i_2 += 1;
        i_2;
    }
    gettimeofday(&mut tvEnd, 0 as *mut libc::c_void);
    timeSubtract(&mut tvDiff, &mut tvEnd, &mut tvBegin);
    printf(
        b"100000 x Good-Thomas: \t %ld.%d\n\0" as *const u8 as *const libc::c_char,
        tvDiff.tv_sec,
        tvDiff.tv_usec,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
