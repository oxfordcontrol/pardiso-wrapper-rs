#![allow(clippy::upper_case_acronyms)]

use super::ffi::*;
use crate::{dylib_path_env, MKLPardisoError};
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use which::which_in;

pub(crate) fn get_mkl_lib_path() -> Option<std::path::PathBuf> {
    let libname = {
        if cfg!(target_os = "windows") {
            "libmkl_rt.dll"
        } else if cfg!(target_os = "macos") {
            "libmkl_rt.dylib"
        } else {
            "libmkl_rt.so"
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
    let mkl_root = std::env::var("MKLROOT").unwrap_or_else(|_| "".to_string());
    let mkl_lib_path = if !mkl_root.is_empty() {
        let mut path = std::path::PathBuf::from(mkl_root.clone());
        path.push("lib");
        path.to_string_lossy().to_string()
    } else {
        "".to_string()
    };

    let search_dirs = [
        mkl_lib_path,                                                         // $MKLROOT/lib
        mkl_root,                                                             // $MKLROOT
        std::env::var("MKL_PARDISO_PATH").unwrap_or_else(|_| "".to_string()), // Environment variable
        "/opt/intel/oneapi/mkl/latest/lib".to_string(),                       // Common system path
        "./".to_string(),
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

fn get_mkl_library() -> Option<Library> {
    // Attempt to get the library path
    let lib_path = get_mkl_lib_path()?;

    // Attempt to load the library, returning None if it fails
    unsafe { Library::new(&lib_path).ok() }
}

pub(crate) fn mkl_ptrs<'a>() -> Result<&'a MKLPardisoPointers<'static>, MKLPardisoError> {
    MKL_SYMBOLS
        .as_ref()
        .ok_or(MKLPardisoError::LibraryLoadFailure)
}

lazy_static! {
    // Store the library separately to ensure it remains loaded
    pub (crate) static ref MKL_LIBRARY: Option<Library> = get_mkl_library();

    // Store the function pointers
    pub(crate) static ref MKL_SYMBOLS: Option<MKLPardisoPointers<'static>> = {
        let lib = MKL_LIBRARY.as_ref()?; // Access the library

        let pardiso: Symbol<PARDISO> = unsafe { lib.get::<PARDISO>(b"pardiso_").ok()? };
        let pardisoinit: Symbol<PARDISOINIT> = unsafe { lib.get::<PARDISOINIT>(b"pardisoinit_").ok()? };
        let mkl_set_num_threads: Symbol<MKL_SET_NUM_THREADS> = unsafe { lib.get::<MKL_SET_NUM_THREADS>(b"mkl_set_num_threads").ok()? };
        let mkl_set_num_threads_local: Symbol<MKL_SET_NUM_THREADS_LOCAL> = unsafe { lib.get::<MKL_SET_NUM_THREADS_LOCAL>(b"mkl_set_num_threads_local").ok()? };
        let mkl_domain_set_num_threads: Symbol<MKL_DOMAIN_SET_NUM_THREADS> = unsafe { lib.get::<MKL_DOMAIN_SET_NUM_THREADS>(b"mkl_domain_set_num_threads").ok()? };
        let mkl_get_max_threads: Symbol<MKL_GET_MAX_THREADS> = unsafe { lib.get::<MKL_GET_MAX_THREADS>(b"mkl_get_max_threads").ok()? };
        let mkl_domain_get_max_threads: Symbol<MKL_DOMAIN_GET_MAX_THREADS> = unsafe { lib.get::<MKL_DOMAIN_GET_MAX_THREADS>(b"mkl_domain_get_max_threads").ok()? };
        let mkl_set_dynamic: Symbol<MKL_SET_DYNAMIC> = unsafe { lib.get::<MKL_SET_DYNAMIC>(b"mkl_set_dynamic").ok()? };

        Some(MKLPardisoPointers {
            pardiso,
            pardisoinit,
            mkl_set_num_threads,
            mkl_set_num_threads_local,
            mkl_domain_set_num_threads,
            mkl_get_max_threads,
            mkl_domain_get_max_threads,
            mkl_set_dynamic,
        })
    };
}

#[test]
fn test_get_set_mkl_threads() {
    use crate::mkl;

    let lib = MKL_LIBRARY.as_ref().unwrap(); // Access the library

    (mkl_ptrs().unwrap().mkl_set_dynamic)(&0_i32);

    // set global thread count
    (mkl_ptrs().unwrap().mkl_set_num_threads)(&2_i32);
    let n = (mkl_ptrs().unwrap().mkl_get_max_threads)();
    assert!(n == 2, "MKL global thread count not set correctly");

    // set by domain
    (mkl_ptrs().unwrap().mkl_domain_set_num_threads)(&3_i32, &mkl::MKL_DOMAIN_PARDISO);
    let n = (mkl_ptrs().unwrap().mkl_domain_get_max_threads)(&mkl::MKL_DOMAIN_PARDISO);
    assert!(n == 3, "MKL domain thread count not set correctly");

    // set local thread count
    (mkl_ptrs().unwrap().mkl_set_num_threads_local)(&4_i32);
    let n = (mkl_ptrs().unwrap().mkl_get_max_threads)();
    assert!(n == 4, "MKL global thread count not set correctly");
}
