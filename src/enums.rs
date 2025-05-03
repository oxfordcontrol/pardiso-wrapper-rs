#[repr(i32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum MatrixType {
    RealStructurallySymmetric = 1,
    RealSymmetricPositiveDefinite = 2,
    #[default]
    RealSymmetricIndefinite = -2,
    ComplexStructurallySymmetric = 3,
    ComplexHermitianPositiveDefinite = 4,
    ComplexHermitianIndefinite = -4,
    ComplexSymmetric = 6,
    RealNonsymmetric = 11,
    ComplexNonsymmetric = 13,
}

impl MatrixType {
    pub fn is_real(&self) -> bool {
        matches!(
            self,
            MatrixType::RealStructurallySymmetric
                | MatrixType::RealSymmetricPositiveDefinite
                | MatrixType::RealSymmetricIndefinite
                | MatrixType::RealNonsymmetric
        )
    }
    pub fn is_complex(&self) -> bool {
        !self.is_real()
    }
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            MatrixType::RealSymmetricPositiveDefinite
                | MatrixType::RealSymmetricIndefinite
                | MatrixType::ComplexSymmetric
        )
    }
    pub fn is_hermitian(&self) -> bool {
        (self.is_symmetric() && self.is_real())
            || matches!(
                self,
                MatrixType::ComplexHermitianPositiveDefinite
                    | MatrixType::ComplexHermitianIndefinite
            )
    }
}

impl std::fmt::Display for MatrixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MatrixType::RealStructurallySymmetric => "Real Structurally Symmetric",
            MatrixType::RealSymmetricPositiveDefinite => "Real Symmetric Positive Definite",
            MatrixType::RealSymmetricIndefinite => "Real Symmetric Indefinite",
            MatrixType::ComplexStructurallySymmetric => "Complex Structurally Symmetric",
            MatrixType::ComplexHermitianPositiveDefinite => "Complex Hermitian Positive Definite",
            MatrixType::ComplexHermitianIndefinite => "Complex Hermitian Indefinite",
            MatrixType::ComplexSymmetric => "Complex Symmetric",
            MatrixType::RealNonsymmetric => "Real Nonsymmetric",
            MatrixType::ComplexNonsymmetric => "Complex Nonsymmetric",
        };
        write!(f, "{}", name)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum Phase {
    #[default]
    Analysis = 11,
    AnalysisNumFact = 12,
    AnalysisNumFactSolveRefine = 13,
    NumFact = 22,
    SelectedInversion = -22,
    NumFactSolveRefine = 23,
    SolveIterativeRefine = 33,
    SolveIterativeRefineOnlyForward = 331,
    SolveIterativeRefineOnlyDiag = 332,
    SolveIterativeRefineOnlyBackward = 333,
    ReleaseLUandMNUM = 0,
    ReleaseAll = -1,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Phase::Analysis => "Analysis",
            Phase::AnalysisNumFact => "Analysis, Numerical Factorization",
            Phase::AnalysisNumFactSolveRefine => {
                "Analysis, Numerical Factorization, and Solve with Refine"
            }
            Phase::NumFact => "Numerical Factorization",
            Phase::SelectedInversion => "Selected Inversion",
            Phase::NumFactSolveRefine => "Numerical Factorization and Solve with Refine",
            Phase::SolveIterativeRefine => "Solve with Iterative Refinement",
            Phase::SolveIterativeRefineOnlyForward => "Solve with Iterative Refinement (Forward)",
            Phase::SolveIterativeRefineOnlyDiag => "Solve with Iterative Refinement (Diagonal)",
            Phase::SolveIterativeRefineOnlyBackward => "Solve with Iterative Refinement (Backward)",
            Phase::ReleaseLUandMNUM => "Release LU and MNUM",
            Phase::ReleaseAll => "Release All",
        };
        write!(f, "{}", name)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum MessageLevel {
    #[default]
    Off = 0,
    On = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum SolverType {
    #[default]
    Direct = 0,
    Iterative = 1,
}
