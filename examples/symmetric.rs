fn main() {
    use pardiso_wrapper::*;

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

    // Create a Pardiso solver instance
    let mut ps = MKLPardisoSolver::new().unwrap(); //requires 'mkl' feature

    // set the matrix type
    ps.set_matrix_type(MatrixType::RealSymmetricIndefinite);

    // Initialize default settings for this matrix type
    ps.pardisoinit().unwrap();

    // be as noisy as possible
    ps.set_message_level(MessageLevel::On);

    // compute the symbolic factorization
    ps.set_phase(Phase::Analysis);
    ps.pardiso(&a, &ia, &ja, &mut [], &mut [], n, 1).unwrap();

    // compute the numeric factorization
    ps.set_phase(Phase::NumFact);
    ps.pardiso(&a, &ia, &ja, &mut b, &mut x, n, m).unwrap();

    // compute the solutions
    ps.set_phase(Phase::SolveIterativeRefine);
    ps.pardiso(&a, &ia, &ja, &mut b, &mut x, n, m).unwrap();

    // Free solver resources
    // Unnecessary (but harmless) since Drop impl does this for you
    ps.set_phase(Phase::ReleaseAll);
    ps.pardiso(&a, &ia, &ja, &mut b, &mut x, n, m).unwrap();
}
