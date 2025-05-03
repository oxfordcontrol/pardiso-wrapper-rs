# pardiso-wrapper-rs
Rust wrapper for MKL and Panua Pardiso

 # PARDISO Wrapper for Rust

 This crate dynamically loads the PARDISO sparse solver library and provides a safe
 Rust interface.  It supports either MKL or Panua Pardiso backends through feature flags:

 - `mkl`: Intel MKL implementation (x86_64 only)
 - `panua`: Panua implementation

 Both options are supported via the common [`PardisoInterface`] trait.

 ### MKL Pardiso

 To enable dynamic linking to [`MKL Pardiso`](https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/),
 the MKL Pardiso libary (e.g. `libmkl_rt.so`) must be on the system library path
 (e.g. on `LD_LIBRARY_PATH` on Linux).    Alternatively, set the `MKLROOT` environment
 variable to the root of the MKL installation or `MKL_PARDISO_PATH` to the location
 of the library.  

 ### Panua Pardiso

 To enable dynamic linking to [`Panua Pardiso`](https://panua.ch/pardiso/),
 the Panua Pardiso library (e.g. `libpardiso.so`) must be on the system library path
 (e.g. on `LD_LIBRARY_PATH` on Linux).  Alternatively, set the `PARDISO_PATH` environment
 variable to the location of the library.

 Panua Pardiso is a commercial solver and requires a separate license.