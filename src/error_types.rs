use num_enum::{FromPrimitive, IntoPrimitive};
use thiserror::Error;

#[derive(Error, Debug, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PanuaPardisoError {
    //panua library error codes
    #[error("Input inconsistent.")]
    InputInconsistent = -1,
    #[error("Not enough memory.")]
    NotEnoughMemory = -2,
    #[error("Reordering problem.")]
    ReorderingProblem = -3,
    #[error("Zero pivot, numerical factorization, or iterative refinement problem.")]
    ZeroPivot = -4,
    #[error("Unclassified (internal) error.")]
    UnclassifiedError = -5,
    #[error("Preordering failed (matrix types 11, 13 only).")]
    PreorderingFailed = -6,
    #[error("Diagonal matrix problem.")]
    DiagonalMatrixProblem = -7,
    #[error("32-bit integer overflow problem.")]
    IntegerOverflow = -8,
    #[error("No license file panua.lic found.")]
    NoLicenseFile = -10,
    #[error("License is expired.")]
    LicenseExpired = -11,
    #[error("Wrong username or hostname.")]
    WrongUsernameOrHostname = -12,
    #[error("Reached maximum number of Krylov-subspace iterations in iterative solver.")]
    MaxKrylovIterations = -100,
    #[error("No sufficient convergence in Krylov-subspace iteration within 25 iterations.")]
    InsufficientConvergence = -101,
    #[error("Error in Krylov-subspace iteration.")]
    KrylovIterationError = -102,
    #[error("Breakdown in Krylov-subspace iteration.")]
    KrylovBreakdown = -103,
    // additional error types for this crate
    #[error("Library load failure.")]
    LibraryLoadFailure = -900,
    // generic license error.  LicenseExpired or NoLicenseFile should be preferred if known
    #[error("Library load failure.")]
    LibraryLicenseFailure = -901,
    #[num_enum(default)]
    #[error("Unrecognized error code.")]
    UnrecognizedError = -999,
}

#[derive(Error, Debug, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum MKLPardisoError {
    //mkl library error codes
    #[error("Input inconsistent.")]
    InputInconsistent = -1,
    #[error("Not enough memory.")]
    NotEnoughMemory = -2,
    #[error("Reordering problem.")]
    ReorderingProblem = -3,
    #[error("Zero pivot, numerical factorization, or iterative refinement problem.")]
    ZeroPivot = -4,
    #[error("Unclassified (internal) error.")]
    UnclassifiedError = -5,
    #[error("Preordering failed (matrix types 11, 13 only).")]
    PreorderingFailed = -6,
    #[error("Diagonal matrix is singular.")]
    DiagonalMatrixSingular = -7,
    #[error("32-bit integer overflow problem.")]
    IntegerOverflow = -8,
    #[error("Not enough memory for OOC.")]
    NotEnoughMemoryOOC = -9,
    #[error("Error opening OOC files.")]
    ErrorOpeningOOCFiles = -10,
    #[error("Read/write error with OOC files.")]
    ReadWriteErrorOOCFiles = -11,
    #[error("pardiso_64 called from 32-bit library.")]
    Pardiso64CalledFrom32BitLibrary = -12,
    #[num_enum(default)]
    // additional error types for this crate
    #[error("Library load failure.")]
    LibraryLoadFailure = -900,
    #[error("Unrecognized error code.")]
    UnrecognizedError = -999,
}

#[derive(Error, Debug)]
pub enum PardisoError {
    #[error("MKL ERROR: {0}")]
    MKL(#[from] MKLPardisoError),
    #[error("PANUA ERROR: {0}")]
    Panua(#[from] PanuaPardisoError),
    #[error("Unknown error.")]
    Unknown,
}
