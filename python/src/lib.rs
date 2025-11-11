use pyo3::prelude::*;

#[pymodule]
pub mod compris {
    use compris::pyo3::{problemo::*, *};

    #[pymodule_export]
    use PyAnnotations;

    #[pymodule_export]
    use PyProblem;

    #[pymodule_export]
    use PyProblems;
}
