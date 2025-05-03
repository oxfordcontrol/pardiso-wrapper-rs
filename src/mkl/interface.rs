use super::loader::*;
use crate::{MKLPardisoError, PardisoData, PardisoError, PardisoInterface};
use std::ffi::c_void;

// as defined in mkl_types.h: #define MKL_DOMAIN_PARDISO  4
pub(crate) const MKL_DOMAIN_PARDISO: i32 = 4;

pub struct MKLPardisoSolver {
    _data: PardisoData,
}

impl PardisoInterface for MKLPardisoSolver {
    fn data(&self) -> &PardisoData {
        &self._data
    }
    fn data_mut(&mut self) -> &mut PardisoData {
        &mut self._data
    }

    fn new() -> Result<Self, PardisoError> {
        if !MKLPardisoSolver::is_loaded() {
            return Err(MKLPardisoError::LibraryLoadFailure)?;
        }
        let data = PardisoData::default();
        Ok(Self { _data: data })
    }

    fn pardisoinit(&mut self) -> Result<(), PardisoError> {
        let ptrs = mkl_ptrs()?;

        let pt = self.data_mut().pt.as_mut_ptr() as *mut c_void;
        let mtype = self.get_matrix_type() as i32;
        let iparm = self.data_mut().iparm.as_mut_ptr();

        (ptrs.pardisoinit)(pt, &mtype, iparm);

        Ok(())
    }

    fn pardiso(
        &mut self,
        a: &[f64],
        ia: &[i32],
        ja: &[i32],
        b: &mut [f64],
        x: &mut [f64],
        n: i32,
        nrhs: i32,
    ) -> Result<(), PardisoError> {
        let ptrs = mkl_ptrs()?;

        let mut error = 0;
        let pt = self.data_mut().pt.as_mut_ptr() as *mut c_void;
        let maxfct = self.data().maxfct;
        let mnum = self.data().mnum;
        let mtype = self.get_matrix_type() as i32;
        let phase = self.data().phase as i32;
        let a = a.as_ptr();
        let ia = ia.as_ptr();
        let ja = ja.as_ptr();
        let b = b.as_mut_ptr();
        let x = x.as_mut_ptr();
        let perm = self.data_mut().perm.as_mut_ptr();
        let iparm = self.data_mut().iparm.as_mut_ptr();
        let msglvl = self.data().msglvl as i32;

        (ptrs.pardiso)(
            pt, &maxfct, &mnum, &mtype, &phase, &n, a, ia, ja, perm, &nrhs, iparm, &msglvl, b, x,
            &mut error,
        );

        if error != 0 {
            let error = MKLPardisoError::from(error);
            return Err(PardisoError::from(error));
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mkl"
    }

    fn is_licensed() -> bool {
        true //MKL doesn't do license checks
    }

    fn is_loaded() -> bool {
        mkl_ptrs().is_ok()
    }

    fn get_num_threads(&self) -> Result<i32, PardisoError> {
        Ok(MKLPardisoSolver::mkl_get_max_threads()?)
    }
}

// additional MKL specific functions
impl MKLPardisoSolver {
    pub fn set_num_threads(&mut self, num_threads: i32) -> Result<i32, PardisoError> {
        Ok(MKLPardisoSolver::mkl_set_num_threads_local(num_threads)?)
    }
    pub fn mkl_set_num_threads(num_threads: i32) -> Result<i32, MKLPardisoError> {
        Ok((mkl_ptrs()?.mkl_set_num_threads)(&num_threads))
    }
    // sets threads for the current execution thread
    // overrides global settings, so we use this for the
    // the default set_num_threads above.  It should
    // be reported correctly by mkl_get_max_threads
    pub fn mkl_set_num_threads_local(num_threads: i32) -> Result<i32, MKLPardisoError> {
        Ok((mkl_ptrs()?.mkl_set_num_threads_local)(&num_threads))
    }
    // sets the number of threads in MKL_DOMAIN_PARDISO only
    pub fn mkl_set_num_threads_pardiso(num_threads: i32) -> Result<i32, MKLPardisoError> {
        Ok((mkl_ptrs()?.mkl_domain_set_num_threads)(
            &num_threads,
            &MKL_DOMAIN_PARDISO,
        ))
    }
    // max threads available to MKL
    pub fn mkl_get_max_threads() -> Result<i32, MKLPardisoError> {
        Ok((mkl_ptrs()?.mkl_get_max_threads)())
    }
    // max threads available to MKL_DOMAIN_PARDISO, possibly limited
    // by environment variables or thread local settings
    pub fn mkl_get_max_threads_pardiso() -> Result<i32, MKLPardisoError> {
        Ok((mkl_ptrs()?.mkl_domain_get_max_threads)(
            &MKL_DOMAIN_PARDISO,
        ))
    }
    pub fn mkl_set_dynamic(dynamic: i32) -> Result<(), MKLPardisoError> {
        (mkl_ptrs()?.mkl_set_dynamic)(&dynamic);
        Ok(())
    }
}

impl Drop for MKLPardisoSolver {
    fn drop(&mut self) {
        self.release();
    }
}
