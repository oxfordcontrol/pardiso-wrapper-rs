#![allow(clippy::upper_case_acronyms)]

//! # PARDISO Wrapper for Rust
//!
//! This crate dynamically loads the PARDISO sparse solver library and provides a safe
//! Rust interface.  It supports either MKL or Panua Pardiso backends through feature flags:
//!
//! - `mkl`: Intel MKL implementation (x86_64 only)
//! - `panua`: Panua implementation
//!
//! Both options are supported via the common [`PardisoInterface`] trait.
//!
//! ### MKL Pardiso
//!
//! To enable dynamic linking to [`MKL Pardiso`](https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/),
//! the MKL Pardiso libary (e.g. `libmkl_rt.so`) must be on the system library path
//! (e.g. on `LD_LIBRARY_PATH` on Linux).    Alternatively, set the `MKLROOT` environment
//! variable to the root of the MKL installation or `MKL_PARDISO_PATH` to the location
//! of the library.  
//!
//! ### Panua Pardiso
//!
//! To enable dynamic linking to [`Panua Pardiso`](https://panua.ch/pardiso/),
//! the Panua Pardiso library (e.g. `libpardiso.so`) must be on the system library path
//! (e.g. on `LD_LIBRARY_PATH` on Linux).  Alternatively, set the `PARDISO_PATH` environment
//! variable to the location of the library.
//!
//! Panua Pardiso is a commercial solver and requires a separate license.
//!
//! ## Example
//! ```rust, no_test
#![doc = include_str!("../examples/symmetric.rs")]
//! ```

mod enums;
pub use enums::*;
mod error_types;
pub use error_types::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "panua")]{
        mod panua;
        pub use panua::PanuaPardisoSolver;
}}

cfg_if::cfg_if! {
    if #[cfg(feature = "mkl")]{
        mod mkl;
        pub use mkl::MKLPardisoSolver;
}}

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct PardisoData {
    pub pt: [isize; 64],
    pub iparm: [i32; 64],
    pub mtype: MatrixType,
    pub solver: SolverType,
    pub phase: Phase,
    pub msglvl: MessageLevel,
    pub maxfct: i32,
    pub mnum: i32,
    pub perm: Vec<i32>,
}

impl Default for PardisoData {
    fn default() -> Self {
        Self {
            pt: [0; 64],
            iparm: [0; 64],
            mtype: MatrixType::default(),
            solver: SolverType::default(),
            phase: Phase::default(),
            msglvl: MessageLevel::default(),
            maxfct: 1,
            mnum: 1,
            perm: vec![],
        }
    }
}

pub trait PardisoInterface {
    // Getters and Setters (default implementations)

    fn get_matrix_type(&self) -> MatrixType {
        self.data().mtype
    }
    fn set_matrix_type(&mut self, mtype: MatrixType) {
        self.data_mut().mtype = mtype;
    }
    fn get_solver(&self) -> SolverType {
        self.data().solver
    }
    fn set_solver(&mut self, solver: SolverType) {
        self.data_mut().solver = solver;
    }
    fn get_phase(&self) -> Phase {
        self.data().phase
    }
    fn set_phase(&mut self, phase: Phase) {
        self.data_mut().phase = phase;
    }
    fn get_message_level(&self) -> MessageLevel {
        self.data().msglvl
    }
    fn set_message_level(&mut self, msglvl: MessageLevel) {
        self.data_mut().msglvl = msglvl;
    }
    fn get_maxfct(&self) -> i32 {
        self.data().maxfct
    }
    fn set_maxfct(&mut self, maxfct: i32) {
        self.data_mut().maxfct = maxfct;
    }
    fn get_mnum(&self) -> i32 {
        self.data().mnum
    }
    fn set_mnum(&mut self, mnum: i32) {
        self.data_mut().mnum = mnum;
    }
    fn get_perm(&self) -> &[i32] {
        self.data().perm.as_slice()
    }
    // : Problem here I think.   Should not own the vector passed
    fn set_perm(&mut self, perm: &[i32]) {
        self.data_mut().perm.resize(perm.len(), 0_i32);
        self.data_mut().perm.copy_from_slice(perm);
    }
    fn get_iparm(&self, i: usize) -> i32 {
        self.data().iparm[i]
    }
    fn get_iparms(&self) -> &[i32; 64] {
        &self.data().iparm
    }
    fn set_iparm(&mut self, i: usize, value: i32) {
        self.data_mut().iparm[i] = value;
    }
    fn get_num_positive_eigenvalues(&self) -> i32 {
        self.data().iparm[21]
    }
    fn get_num_negative_eigenvalues(&self) -> i32 {
        self.data().iparm[22]
    }

    // NB: implementors should also implement Drop and call
    // release() in the drop method.
    fn release(&mut self) {
        // Set the phase to release all resources
        self.set_phase(crate::Phase::ReleaseAll);

        // call with dummies since we are releasing resource only
        let a: Vec<f64> = vec![];
        let ia: Vec<i32> = vec![];
        let ja: Vec<i32> = vec![];
        let mut b: Vec<f64> = vec![];
        let mut x: Vec<f64> = vec![];

        // Call Pardiso to release resources, ignoring any errors
        let _ = self.pardiso(&a, &ia, &ja, &mut b, &mut x, 0, 0);
    }

    // MKL/Panua specific functions

    fn data(&self) -> &PardisoData;
    fn data_mut(&mut self) -> &mut PardisoData;
    fn name(&self) -> &'static str;

    fn new() -> Result<Self, PardisoError>
    where
        Self: Sized;

    fn pardisoinit(&mut self) -> Result<(), PardisoError>;

    #[allow(clippy::too_many_arguments)]
    fn pardiso(
        &mut self,
        a: &[f64],
        ia: &[i32],
        ja: &[i32],
        b: &mut [f64],
        x: &mut [f64],
        n: i32,
        nrhs: i32,
    ) -> Result<(), PardisoError>;

    fn is_licensed() -> bool
    where
        Self: Sized;
    fn is_loaded() -> bool
    where
        Self: Sized;
    fn is_available() -> bool
    where
        Self: Sized,
    {
        Self::is_licensed() && Self::is_loaded()
    }

    // NB: there is no set_num_threads in this trait since
    // Panua does not allow for configurable thread counts
    // other than via the OMP_NUM_THREADS environment variable
    fn get_num_threads(&self) -> Result<i32, PardisoError>;
}

// default dylib env by platform
#[allow(dead_code)] // if no features are set
pub(crate) fn dylib_path_env() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            "DYLD_LIBRARY_PATH"
        } else if #[cfg(target_os = "windows")] {
            "PATH"
        } else {
            "LD_LIBRARY_PATH" // linux/unknown platform
        }
    }
}
