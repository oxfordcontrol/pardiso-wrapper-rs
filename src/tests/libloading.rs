#[cfg(feature = "panua")]
#[test]
fn test_panua_libloading() {
    use crate::panua::loader::PANUA_LIBRARY;
    use crate::panua::loader::PANUA_SYMBOLS;

    // Check if the library has been loaded
    assert!(
        PANUA_LIBRARY.is_some(),
        "Panua library not loaded successfully"
    );
    assert!(
        PANUA_SYMBOLS.is_some(),
        "Panua pointers not loaded successfully"
    );
}

#[cfg(feature = "panua")]
#[test]
fn test_is_panua_licensed() {
    use crate::{PanuaPardisoSolver, PardisoInterface};
    assert!(
        PanuaPardisoSolver::is_licensed(),
        "Panua library license not working"
    );
}

#[cfg(feature = "mkl")]
#[test]
fn test_mkl_libloading() {
    use crate::mkl::loader::MKL_LIBRARY;
    use crate::mkl::loader::MKL_SYMBOLS;

    // Check if the library has been loaded
    assert!(MKL_LIBRARY.is_some(), "MKL library not loaded successfully");
    assert!(
        MKL_SYMBOLS.is_some(),
        "MKL pointers not loaded successfully"
    );
}

#[cfg(feature = "panua")]
#[test]
fn test_get_panua_lib_path() {
    use crate::panua::loader::get_panua_lib_path;

    // Attempt to get the library path
    if let Some(path) = get_panua_lib_path() {
        assert!(
            std::path::Path::new(&path).exists(),
            "Library path should exist"
        );
    } else {
        panic!("Panua library path could not be found");
    }
}

#[cfg(feature = "mkl")]
#[test]
fn test_get_mkl_lib_path() {
    use crate::mkl::loader::get_mkl_lib_path;

    // Attempt to get the library path
    if let Some(path) = get_mkl_lib_path() {
        assert!(
            std::path::Path::new(&path).exists(),
            "Library path should exist"
        );
    } else {
        panic!("MKL library path could not be found");
    }
}
