fn main() {
    // `mkl` only works on x86 and x86_64 architectures
    if cfg!(feature = "mkl") {
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

        if target_arch != "x86_64" {
            panic!("{} build error: the `mkl` feature is only supported for x86_64.  This platform is {}.", pkg_name, target_arch);
        }

        // mkl requires libm to be linked
        println!("cargo:rustc-link-lib=m");
    }
}
