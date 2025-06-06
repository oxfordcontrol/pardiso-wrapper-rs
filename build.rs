fn main() {
    if cfg!(feature = "mkl") {
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

        let is_docs = cfg!(feature = "docs") || std::env::var("DOCS_RS").is_ok();

        // Skip the check when generating documentation
        if !is_docs && target_arch != "x86_64" {
            panic!("{pkg_name} build error: the `mkl` feature is only supported for x86_64.  This platform is {target_arch}.");
        }

        if !cfg!(target_os = "windows") {
            // mkl requires libm on osx/linux
            println!("cargo:rustc-link-lib=m");
        }
    }
}
