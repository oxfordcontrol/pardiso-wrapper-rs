#![allow(clippy::upper_case_acronyms)]

use super::ffi::*;
use crate::{dylib_path_env, PanuaPardisoError, PanuaPardisoSolver, PardisoData};
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use which::which_in;

pub(crate) fn get_panua_lib_path() -> Option<std::path::PathBuf> {
    let libname = {
        if cfg!(target_os = "windows") {
            "libpardiso.dll"
        } else if cfg!(target_os = "macos") {
            "libpardiso.dylib"
        } else {
            "libpardiso.so"
        }
    };

    // Look first in LD_LIBRARY_PATH
    let ld_library_path = std::env::var(dylib_path_env()).unwrap_or_else(|_| "".to_string());

    // Search for libpardiso.so in LD_LIBRARY_PATH and other standard paths
    if let Ok(ldpath) = which_in(
        libname,
        Some(ld_library_path),
        std::env::current_dir().ok()?,
    ) {
        return Some(ldpath);
    }

    // If not found, search in likely directories
    let search_dirs = [
        std::env::var("PARDISO_PATH").unwrap_or_else(|_| "".to_string()), // Environment variable
        "/usr/lib/".to_string(),                                          // Common system path
        "/usr/local/lib/".to_string(),                                    // Common local path
        ".".to_string(),
    ];

    let search_dirs = std::env::join_paths(search_dirs.iter())
        .ok()?
        .to_string_lossy()
        .to_string();

    // Use which_in to search for libpardiso.so in the specified directories
    if let Ok(path) = which_in(libname, Some(search_dirs), std::env::current_dir().ok()?) {
        return Some(path);
    }

    None
}

pub(crate) fn get_panua_library() -> Option<Library> {
    // Attempt to get the library path
    let lib_path = get_panua_lib_path()?;

    // Attempt to load the library, returning None if it fails
    unsafe { Library::new(&lib_path).ok() }
}

pub(crate) fn panua_ptrs<'a>() -> Result<&'a PanuaPardisoPointers<'static>, PanuaPardisoError> {
    PANUA_SYMBOLS
        .as_ref()
        .ok_or(PanuaPardisoError::LibraryLoadFailure)
}

pub(crate) fn panua_is_licensed() -> bool {
    *PANUA_IS_LICENSED
}

lazy_static! {

    // Store the library separately to ensure it remains loaded
    pub (crate) static ref PANUA_LIBRARY: Option<Library> = get_panua_library();

    // Store the function pointers
    pub(crate) static ref PANUA_SYMBOLS: Option<PanuaPardisoPointers<'static>> = {
        let lib = PANUA_LIBRARY.as_ref()?; // Access the library

        let pardiso: Symbol<PARDISO> = unsafe { lib.get::<PARDISO>(b"pardiso_").ok()? };
        let pardisoinit: Symbol<PARDISOINIT> = unsafe { lib.get::<PARDISOINIT>(b"pardisoinit_").ok()? };
        let pardiso_chkmatrix: Symbol<PARDISO_CHKMATRIX> = unsafe { lib.get::<PARDISO_CHKMATRIX>(b"pardiso_chkmatrix_").ok()? };
        let pardiso_chkvec: Symbol<PARDISO_CHKVEC> = unsafe { lib.get::<PARDISO_CHKVEC>(b"pardiso_chkvec_").ok()? };
        let pardiso_printstats: Symbol<PARDISO_PRINTSTATS> = unsafe { lib.get::<PARDISO_PRINTSTATS>(b"pardiso_printstats_").ok()? };

        Some(PanuaPardisoPointers {
            pardiso,
            pardisoinit,
            pardiso_chkmatrix,
            pardiso_chkvec,
            pardiso_printstats,
        })
    };

    // Record licensing state
    static ref PANUA_IS_LICENSED: bool = {

        if panua_ptrs().is_ok() {

            // call directly into the library with fake data
            let data = &mut PardisoData::default();
            let dparm = &mut [0.0; 64];
            let mtype = crate::MatrixType::default();
            let solver = crate::SolverType::default();

           let r = PanuaPardisoSolver::pardisoinit_impl(data, dparm, mtype, solver);

            match r {
                Ok(_) => true,
                Err(PanuaPardisoError::NoLicenseFile) => false,
                Err(PanuaPardisoError::LicenseExpired) => false,
                Err(_) => panic!("Unexpected error while checking license"),
            }
        }
        else {
            false
        }
    };
}

#[test]
fn test_panua_check_fcns() {
    // Matrix data from symmetric indefinite example
    let n: i32 = 4; // Number of equations
    let m: i32 = 3; // Number of right-hand sides
    let a = [1.0, -2.0, 3.0, 5.0, 1.0, 2.0, 4.0, -7.0, 5.0];
    let ia = [1, 4, 7, 9, 10];
    let ja = [1, 3, 4, 2, 3, 4, 3, 4, 4];
    let mtype = crate::MatrixType::RealSymmetricIndefinite as i32;

    // Generate some right hand side data
    let b: Vec<f64> = (0..(n * m)).map(|x| x as f64).collect();

    let mut error = 0_i32;

    (panua_ptrs().unwrap().pardiso_chkmatrix)(
        &mtype,
        &n,
        a.as_ptr(),
        ia.as_ptr(),
        ja.as_ptr(),
        &mut error,
    );
    assert_eq!(error, 0, "Matrix check failed with error code: {}", error);

    (panua_ptrs().unwrap().pardiso_chkvec)(
        &n as *const i32,
        &m as *const i32,
        b.as_ptr(),
        &mut error as *mut i32,
    );
    assert_eq!(error, 0, "Vector check failed with error code: {}", error);

    (panua_ptrs().unwrap().pardiso_printstats)(
        &mtype,
        &n,
        a.as_ptr(),
        ia.as_ptr(),
        ja.as_ptr(),
        &m,
        b.as_ptr(),
        &mut error,
    );

    assert_eq!(error, 0, "Print stats failed with error code: {}", error);
}
