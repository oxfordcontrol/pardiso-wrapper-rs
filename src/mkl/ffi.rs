#![allow(non_camel_case_types)]

use libloading::Symbol;
use std::ffi::{c_int, c_void};

#[derive(Debug)]
pub(crate) struct MKLPardisoPointers<'a> {
    pub pardiso: Symbol<'a, PARDISO>,
    pub pardisoinit: Symbol<'a, PARDISOINIT>,
    pub mkl_set_num_threads: Symbol<'a, MKL_SET_NUM_THREADS>,
    pub mkl_set_num_threads_local: Symbol<'a, MKL_SET_NUM_THREADS_LOCAL>,
    pub mkl_domain_set_num_threads: Symbol<'a, MKL_DOMAIN_SET_NUM_THREADS>,
    pub mkl_get_max_threads: Symbol<'a, MKL_GET_MAX_THREADS>,
    pub mkl_domain_get_max_threads: Symbol<'a, MKL_DOMAIN_GET_MAX_THREADS>,
    #[allow(dead_code)]
    pub mkl_set_dynamic: Symbol<'a, MKL_SET_DYNAMIC>,
}

// function signatures differ between MKL and Panua, so
// they are defined separately.  Note in particular that
// the MKL version does not have dparm

pub(crate) type PARDISO = extern "C" fn(
    pt: *mut c_void,
    maxfct: *const i32,
    mnum: *const i32,
    mtype: *const i32,
    phase: *const i32,
    n: *const i32,
    a: *const f64,
    ia: *const i32,
    ja: *const i32,
    perm: *mut i32,
    nrhs: *const i32,
    iparm: *mut i32,
    msglvl: *const i32,
    b: *mut f64,
    x: *mut f64,
    error: *mut i32,
);

pub(crate) type PARDISOINIT = extern "C" fn(pt: *mut c_void, mtype: *const i32, iparm: *mut i32);

// MKL C documentation says that the thread get/set functions take c_int, which
// appears to be wrong since they call into fortran functions that expect c_int*

pub(crate) type MKL_SET_NUM_THREADS = extern "C" fn(nthreads: *const c_int) -> c_int;

pub(crate) type MKL_SET_NUM_THREADS_LOCAL = extern "C" fn(nthreads: *const c_int) -> c_int;

pub(crate) type MKL_DOMAIN_SET_NUM_THREADS =
    extern "C" fn(nthreads: *const c_int, domain: *const c_int) -> c_int;

pub(crate) type MKL_GET_MAX_THREADS = extern "C" fn() -> c_int;

pub(crate) type MKL_DOMAIN_GET_MAX_THREADS = extern "C" fn(domain: *const c_int) -> c_int;

pub(crate) type MKL_SET_DYNAMIC = extern "C" fn(dynamic: *const c_int);
