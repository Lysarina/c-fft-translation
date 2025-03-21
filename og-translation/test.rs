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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct complex_t {
    pub re: libc::c_double,
    pub im: libc::c_double,
}
pub type complex = complex_t;
unsafe fn main_0() -> libc::c_int {
    let mut input1: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as libc::c_ulong)
            .wrapping_mul(30 as libc::c_int as libc::c_ulong),
    ) as *mut complex;
    let mut input2: *mut complex = malloc(
        (::core::mem::size_of::<complex_t>() as libc::c_ulong)
            .wrapping_mul(30 as libc::c_int as libc::c_ulong),
    ) as *mut complex;
    let mut result1: *mut complex = 0 as *mut complex;
    let mut result2: *mut complex = 0 as *mut complex;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        (*input1.offset(i as isize)).re = i as libc::c_double;
        (*input1.offset(i as isize)).im = 0.0f64;
        (*input2.offset(i as isize)).re = i as libc::c_double;
        (*input2.offset(i as isize)).im = 0.0f64;
        i += 1;
        i;
    }
    result1 = FFT_CooleyTukey(
        input1,
        30 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
    );
    result2 = FFT_GoodThomas(
        input2,
        30 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
    );
    printf(
        b"Index \t Cooley-Tukey Output \t \t Good-Thomas Output \n\0" as *const u8
            as *const libc::c_char,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 30 as libc::c_int {
        printf(
            b"%d: \t %f + %fi \t %f + %fi \n\0" as *const u8 as *const libc::c_char,
            i_0,
            (*result1.offset(i_0 as isize)).re,
            (*result1.offset(i_0 as isize)).im,
            (*result2.offset(i_0 as isize)).re,
            (*result2.offset(i_0 as isize)).im,
        );
        i_0 += 1;
        i_0;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
