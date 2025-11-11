use super::{super::super::depict::*, problem::*};

use {depiction::*, problemo::*, pyo3::prelude::*, std::io};

//
// PyProblems
//

/// Problems.
#[pyclass(name = "Problems")]
pub struct PyProblems {
    /// Inner problems.
    pub inner: Problems,
}

#[pymethods]
impl PyProblems {
    /// Current number of problems.
    pub fn __len__(&self) -> usize {
        self.inner.count()
    }

    /// Iterator.
    pub fn __iter__(self_: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        Ok(self_)
    }

    /// Next problem.
    pub fn __next__<'py>(&mut self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyProblem>>> {
        Ok(match self.next() {
            Some(problem) => Some(Bound::new(py, PyProblem::from(problem))?),
            None => None,
        })
    }

    /// Format.
    pub fn __format__(&self, specification: Option<&str>) -> PyResult<String> {
        self.py_format(specification)
    }

    /// As string.
    pub fn __str__(&self) -> PyResult<String> {
        self.py_str()
    }
}

impl PyProblems {
    /// Next.
    pub fn next(&mut self) -> Option<Problem> {
        if !self.inner.is_empty() { Some(self.inner.problems.remove(0)) } else { None }
    }
}

impl ToDepiction for PyProblems {
    fn to_depiction(&self, context: &DepictionContext) -> io::Result<String> {
        self.inner.annotated_depiction().to_depiction(context)
    }
}

impl Default for PyProblems {
    fn default() -> Self {
        Problems::default().into()
    }
}

impl From<Problems> for PyProblems {
    fn from(inner: Problems) -> Self {
        Self { inner }
    }
}
