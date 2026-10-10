#![allow(clippy::upper_case_acronyms)]

use super::ffi::*;
use crate::{dylib_path_env, MKLPardisoError};
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
#[cfg(not(target_os = "windows"))]
use which::which_in;

cfg_if::cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        fn force_libm_linking() {
            // this kludgy function exists to force libm linking
            // even when building examples, which otherwise won't
            // link to libm despite aggressive lobbying from both
            // cargo.toml and build.rs
            #[link(name = "m")]
            extern "C" {
                fn log(val: f64) -> f64;
            }
            // this prevents the compiler from optimizing
            // away this otherwise useless function call
            use std::hint::black_box;
            black_box(unsafe { log(black_box(1.0)) });
        }
    } else {
        fn force_libm_linking() {
            // no-op, libm doesn't exist on windows
        }
    }
}

pub(crate) fn get_mkl_lib_path() -> Option<std::path::PathBuf> {
    force_libm_linking();

    #[cfg(target_os = "windows")]
    {
        get_windows_mkl_lib_path()
    }

    #[cfg(not(target_os = "windows"))]
    {
        get_unix_mkl_lib_path()
    }
}

#[cfg(target_os = "windows")]
fn get_windows_mkl_lib_path() -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    let mut dirs: Vec<PathBuf> = std::env::var_os(dylib_path_env())
        .map(|path| std::env::split_paths(&path).collect())
        .unwrap_or_default();

    if let Some(root) = std::env::var_os("MKLROOT") {
        let root = PathBuf::from(root);
        dirs.push(root.join("bin"));
        dirs.push(root);
    }
    if let Some(path) = std::env::var_os("MKL_PARDISO_PATH") {
        dirs.push(PathBuf::from(path));
    }
    dirs.push(std::env::current_dir().ok()?);

    find_windows_mkl_lib(dirs)
}

#[cfg(target_os = "windows")]
fn find_windows_mkl_lib(
    dirs: impl IntoIterator<Item = std::path::PathBuf>,
) -> Option<std::path::PathBuf> {
    for dir in dirs {
        let direct = dir.join("mkl_rt.dll");
        if direct.is_file() {
            return Some(direct);
        }

        // oneMKL releases can install an ABI-versioned DLL (for example mkl_rt.2.dll).
        let versioned = std::fs::read_dir(&dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let version = name
                    .strip_prefix("mkl_rt.")?
                    .strip_suffix(".dll")?
                    .parse::<u32>()
                    .ok()?;
                entry.path().is_file().then_some((version, entry.path()))
            })
            .max_by_key(|(version, _)| *version);
        if let Some((_, path)) = versioned {
            return Some(path);
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn get_unix_mkl_lib_path() -> Option<std::path::PathBuf> {
    let libname = if cfg!(target_os = "macos") {
        "libmkl_rt.dylib"
    } else {
        "libmkl_rt.so"
    };

    let ld_library_path = std::env::var(dylib_path_env()).unwrap_or_default();
    if let Ok(ldpath) = which_in(
        libname,
        Some(ld_library_path),
        std::env::current_dir().ok()?,
    ) {
        return Some(ldpath);
    }

    let mkl_root = std::env::var("MKLROOT").unwrap_or_default();
    let mkl_lib_path = if !mkl_root.is_empty() {
        let mut path = std::path::PathBuf::from(mkl_root.clone());
        path.push("lib");
        path.to_string_lossy().to_string()
    } else {
        String::new()
    };

    let search_dirs = [
        mkl_lib_path,
        mkl_root,
        std::env::var("MKL_PARDISO_PATH").unwrap_or_default(),
        "/opt/intel/oneapi/mkl/latest/lib".to_string(),
        "./".to_string(),
    ];

    let search_dirs = std::env::join_paths(search_dirs.iter())
        .ok()?
        .to_string_lossy()
        .to_string();

    which_in(libname, Some(search_dirs), std::env::current_dir().ok()?).ok()
}

#[cfg(all(test, target_os = "windows"))]
mod windows_path_tests {
    use super::find_windows_mkl_lib;

    #[test]
    fn finds_unversioned_and_versioned_dlls() {
        let dir = std::env::temp_dir().join(format!(
            "pardiso-wrapper-mkl-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&dir).unwrap();

        let versioned = dir.join("mkl_rt.2.dll");
        std::fs::write(&versioned, []).unwrap();
        assert_eq!(find_windows_mkl_lib([dir.clone()]), Some(versioned));

        let unversioned = dir.join("mkl_rt.dll");
        std::fs::write(&unversioned, []).unwrap();
        assert_eq!(find_windows_mkl_lib([dir.clone()]), Some(unversioned));

        std::fs::remove_dir_all(dir).unwrap();
    }
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

    let _lib = MKL_LIBRARY.as_ref().unwrap(); // Access the library

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
