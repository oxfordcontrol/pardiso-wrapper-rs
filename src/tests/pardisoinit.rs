#[cfg(feature = "panua")]
#[test]
fn test_panua_pardisoinit() {
    use crate::*;
    let mut ps = PanuaPardisoSolver::new().unwrap();
    ps.pardisoinit().unwrap();
}

#[cfg(feature = "mkl")]
#[test]
fn test_mkl_pardisoinit() {
    use crate::*;
    let mut ps = MKLPardisoSolver::new().unwrap();
    ps.pardisoinit().unwrap();
}
