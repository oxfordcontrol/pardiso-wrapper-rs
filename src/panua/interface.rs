use super::loader::*;
use crate::enums::{MatrixType, SolverType};
use crate::{PanuaPardisoError, PardisoData, PardisoError, PardisoInterface};
use std::ffi::c_void;

pub struct PanuaPardisoSolver {
    _data: PardisoData,
    _dparm: [f64; 64],
}

impl PanuaPardisoSolver {
    pub(crate) fn pardisoinit_impl(
        data: &mut PardisoData,
        dparm: &mut [f64; 64],
        mtype: MatrixType,
        solver: SolverType,
    ) -> Result<(), PanuaPardisoError> {
        let ptrs = panua_ptrs()?;

        let mut error: i32 = 0;
        let pt = data.pt.as_mut_ptr() as *mut c_void;
        let mtype = mtype as i32;
        let solver = solver as i32;
        let iparm = data.iparm.as_mut_ptr();
        let dparm = dparm.as_mut_ptr();

        (ptrs.pardisoinit)(pt, &mtype, &solver, iparm, dparm, &mut error);

        if error != 0 {
            return Err(PanuaPardisoError::from(error));
        }
        Ok(())
    }
}

impl PanuaPardisoSolver {
    pub fn get_dparm(&self, i: usize) -> f64 {
        self._dparm[i]
    }
    pub fn get_dparms(&self) -> &[f64; 64] {
        &self._dparm
    }
    pub fn set_dparm(&mut self, i: usize, value: f64) {
        self._dparm[i] = value;
    }
}

impl PardisoInterface for PanuaPardisoSolver {
    fn data(&self) -> &PardisoData {
        &self._data
    }
    fn data_mut(&mut self) -> &mut PardisoData {
        &mut self._data
    }

    fn new() -> Result<Self, PardisoError> {
        if !PanuaPardisoSolver::is_loaded() {
            return Err(PanuaPardisoError::LibraryLoadFailure)?;
        }
        if !PanuaPardisoSolver::is_licensed() {
            return Err(PanuaPardisoError::LibraryLicenseFailure)?;
        }

        let data = PardisoData::default();
        let dparm = [0.0; 64];
        Ok(Self {
            _data: data,
            _dparm: dparm,
        })
    }

    fn pardisoinit(&mut self) -> Result<(), PardisoError> {
        let mtype = self.get_matrix_type();
        let solver = self.get_solver();
        let data = &mut self._data;
        let dparm = &mut self._dparm;

        Ok(PanuaPardisoSolver::pardisoinit_impl(
            data, dparm, mtype, solver,
        )?)
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
        let ptrs = panua_ptrs()?;

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
        let dparm = self._dparm.as_mut_ptr();

        (ptrs.pardiso)(
            pt, &maxfct, &mnum, &mtype, &phase, &n, a, ia, ja, perm, &nrhs, iparm, &msglvl, b, x,
            &mut error, dparm,
        );

        if error != 0 {
            let error = PanuaPardisoError::from(error);
            return Err(PardisoError::from(error));
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "panua"
    }

    fn is_licensed() -> bool {
        crate::panua::interface::panua_is_licensed()
    }

    fn is_loaded() -> bool {
        panua_ptrs().is_ok()
    }

    fn get_num_threads(&self) -> Result<i32, PardisoError> {
        Ok(self.data().iparm[2])
    }
}

// additional Panua specific functions
impl PanuaPardisoSolver {
    pub fn pardiso_chkmatrix(
        &self,
        mtype: MatrixType,
        n: i32,
        a: &[f64],
        ia: &[i32],
        ja: &[i32],
    ) -> Result<(), PanuaPardisoError> {
        let ptrs = panua_ptrs()?;

        let mut error = 0;
        let mtype = mtype as i32;
        let a = a.as_ptr();
        let ia = ia.as_ptr();
        let ja = ja.as_ptr();

        (ptrs.pardiso_chkmatrix)(&mtype, &n, a, ia, ja, &mut error);

        if error != 0 {
            return Err(PanuaPardisoError::from(error));
        }
        Ok(())
    }

    pub fn pardiso_chkvec(&self, n: i32, nrhs: i32, b: &[f64]) -> Result<(), PanuaPardisoError> {
        let ptrs = panua_ptrs()?;

        let mut error = 0;
        let b = b.as_ptr();

        (ptrs.pardiso_chkvec)(&n, &nrhs, b, &mut error);

        if error != 0 {
            return Err(PanuaPardisoError::from(error));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pardiso_printstats(
        &self,
        mtype: MatrixType,
        n: i32,
        a: &[f64],
        ia: &[i32],
        ja: &[i32],
        nrhs: i32,
        b: &[f64],
    ) -> Result<(), PanuaPardisoError> {
        let ptrs = panua_ptrs()?;

        let mut error = 0;
        let mtype = mtype as i32;
        let a = a.as_ptr();
        let ia = ia.as_ptr();
        let ja = ja.as_ptr();
        let b = b.as_ptr();

        (ptrs.pardiso_printstats)(&mtype, &n, a, ia, ja, &nrhs, b, &mut error);

        if error != 0 {
            return Err(PanuaPardisoError::from(error));
        }
        Ok(())
    }
}

impl Drop for PanuaPardisoSolver {
    fn drop(&mut self) {
        self.release();
    }
}
