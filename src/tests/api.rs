#[cfg(feature = "panua")]
#[test]
fn test_get_set_dparm() {
    use crate::*;
    let mut solver = PanuaPardisoSolver::new().unwrap();

    // Set a value in the dparm array
    solver.set_dparm(0, std::f64::consts::PI);
    solver.set_dparm(1, std::f64::consts::E);

    // Verify the values were set correctly
    assert_eq!(solver.get_dparm(0), std::f64::consts::PI);
    assert_eq!(solver.get_dparm(1), std::f64::consts::E);
}

#[cfg(feature = "panua")]
#[test]
fn test_get_dparms() {
    use crate::*;
    let mut solver = PanuaPardisoSolver::new().unwrap();

    // Set some values in the dparm array
    solver.set_dparm(0, std::f64::consts::PI);
    solver.set_dparm(1, std::f64::consts::E);
    solver.set_dparm(2, std::f64::consts::TAU);

    // Retrieve the entire dparm array
    let dparms = solver.get_dparms();

    // Verify the values in the array
    assert_eq!(dparms[0], std::f64::consts::PI);
    assert_eq!(dparms[1], std::f64::consts::E);
    assert_eq!(dparms[2], std::f64::consts::TAU);
}

#[cfg(feature = "mkl")]
#[test]
fn test_get_set_num_threads_mkl() {
    use crate::*;
    let mut ps = MKLPardisoSolver::new().unwrap();

    ps.pardisoinit().unwrap();
    ps.set_num_threads(4).unwrap();
    let n = ps.get_num_threads().unwrap();
    assert!(n == 4, "Number of threads should be greater than 0");
}
