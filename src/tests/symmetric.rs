#[cfg(any(feature = "mkl", feature = "panua"))]
fn test_symmetric(ps: &mut dyn crate::PardisoInterface) {
    use crate::*;
    use approx::assert_abs_diff_eq;

    // Parameters
    let n: i32 = 4; // Number of equations
    let m: i32 = 3; // Number of right-hand sides

    // Define a 4x4 symmetric matrix A
    // [ 1. 0 -2  3
    //   0  5  1  2
    //  -2  1  4 -7
    //   3  2 -7  5 ]

    // triangular matrix data (1-based indexing, CSR format)
    let a = vec![1.0, -2.0, 3.0, 5.0, 1.0, 2.0, 4.0, -7.0, 5.0];
    let ia = vec![1, 4, 7, 9, 10];
    let ja = vec![1, 3, 4, 2, 3, 4, 3, 4, 4];

    // Generate some right hand side data
    let mut b: Vec<f64> = (0..(n * m)).map(|x| x as f64).collect();
    let mut x = vec![0.0; (n * m) as usize]; // Solution vector

    // set the matrix type
    ps.set_matrix_type(MatrixType::RealSymmetricIndefinite);

    // Initialize default settings for this matrix type
    ps.pardisoinit().unwrap();

    // compute the symbolic factorization
    ps.set_phase(Phase::Analysis);
    ps.pardiso(&a, &ia, &ja, &mut [], &mut [], n, 1).unwrap();

    // compute the numeric factorization
    ps.set_phase(Phase::NumFact);
    ps.pardiso(&a, &ia, &ja, &mut b, &mut x, n, m).unwrap();

    println!(
        "The matrix has {} positive and {} negative eigenvalues",
        ps.get_num_positive_eigenvalues(),
        ps.get_num_negative_eigenvalues()
    );

    assert!(ps.get_num_positive_eigenvalues() == 3);
    assert!(ps.get_num_negative_eigenvalues() == 1);

    // compute the solutions
    ps.set_phase(Phase::SolveIterativeRefine);
    ps.pardiso(&a, &ia, &ja, &mut b, &mut x, n, m).unwrap();

    let xans = vec![
        16.0, -0.0, 5.0, -2.0, 207.2, -5.6, 72.2, -19.6, 398.4, -11.2, 139.4, -37.2,
    ];

    // Check if xans and x are approximately equal
    assert_abs_diff_eq!(x.as_slice(), xans.as_slice(), epsilon = 1e-6);

    // Free solver resources
    // Unnecessary (but harmless) since Drop impl does this for you
    ps.set_phase(crate::Phase::ReleaseAll);
    ps.pardiso(&a, &ia, &ja, &mut b, &mut x, n, m).unwrap();
}

#[cfg(feature = "mkl")]
#[test]
fn test_symmetric_mkl() {
    use crate::*;
    // Create an MKL Pardiso solver instance
    let mut ps = MKLPardisoSolver::new().unwrap();
    test_symmetric(&mut ps);
}

#[cfg(feature = "panua")]
#[test]
fn test_symmetric_panua() {
    use crate::*;
    // Create a Panua Pardiso solver instance
    let mut ps = PanuaPardisoSolver::new().unwrap();
    test_symmetric(&mut ps);
}
