#![allow(non_camel_case_types)]

use libloading::Symbol;
use std::ffi::c_void;

#[derive(Debug)]
pub(crate) struct PanuaPardisoPointers<'a> {
    pub pardiso: Symbol<'a, PARDISO>,
    pub pardisoinit: Symbol<'a, PARDISOINIT>,
    pub pardiso_chkmatrix: Symbol<'a, PARDISO_CHKMATRIX>,
    pub pardiso_chkvec: Symbol<'a, PARDISO_CHKVEC>,
    pub pardiso_printstats: Symbol<'a, PARDISO_PRINTSTATS>,
}

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
    dparm: *mut f64,
);

pub(crate) type PARDISOINIT = extern "C" fn(
    pt: *mut c_void,
    mtype: *const i32,
    solver: *const i32,
    iparm: *mut i32,
    dparm: *mut f64,
    error: *mut i32,
);

pub(crate) type PARDISO_CHKMATRIX = extern "C" fn(
    mtype: *const i32,
    n: *const i32,
    a: *const f64,
    ia: *const i32,
    ja: *const i32,
    error: *mut i32,
);

pub(crate) type PARDISO_CHKVEC =
    extern "C" fn(n: *const i32, nrhs: *const i32, b: *const f64, error: *mut i32);

pub(crate) type PARDISO_PRINTSTATS = extern "C" fn(
    mtype: *const i32,
    n: *const i32,
    a: *const f64,
    ia: *const i32,
    ja: *const i32,
    nrhs: *const i32,
    b: *const f64,
    error: *mut i32,
);
