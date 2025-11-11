use super::{
    super::{
        super::{annotate::*, depict::*},
        annotate::*,
    },
    problems::*,
};

use {
    ::problemo::*,
    depiction::*,
    kutil::pyo3::*,
    pyo3::{exceptions::*, prelude::*},
    std::io,
};

//
// PyProblem
//

/// Problem.
#[pyclass(name = "Problem", extends=PyException)]
pub struct PyProblem {
    /// Inner problem.
    pub inner: Option<Problem>,
}

#[pymethods]
impl PyProblem {
    /// Annotations.
    #[getter]
    pub fn annotations<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAnnotations>>> {
        Ok(match self.get_annotations() {
            Some(annotations) => Some(Bound::new(py, PyAnnotations::from(annotations))?),
            None => None,
        })
    }

    /// As problems.
    pub fn as_problems(&mut self) -> PyProblems {
        match self.inner.take() {
            Some(problem) => problem.into_problems().into(),
            None => Default::default(),
        }
    }

    /// As unique problems.
    pub fn as_unique_problems(&mut self) -> PyProblems {
        match self.inner.take() {
            Some(problem) => problem.into_problems().into_unique().into(),
            None => Default::default(),
        }
    }

    /// Format.
    pub fn __format__(&self, specification: Option<&str>) -> PyResult<String> {
        if let Some(specification) = specification {
            let mut tags = FormatSpecificationTags::from(specification);
            if tags.remove("annotate") {
                return AnnotatedPyProblem::from(self).py_format(tags.pack().as_deref());
            }
        }

        self.py_format(specification)
    }

    /// As string.
    pub fn __str__(&self) -> PyResult<String> {
        self.py_str()
    }
}

impl PyProblem {
    /// Constructor.
    pub fn new_err(inner: Problem) -> PyResult<PyErr> {
        Python::attach(|py| {
            let problem = Bound::new(py, Self { inner: Some(inner) })?;
            Ok(PyErr::from_value(problem.into_any()))
        })
    }

    /// Annotations.
    pub fn get_annotations(&self) -> Option<Annotations> {
        self.inner.as_ref().and_then(|problem| problem.attachment_of_type::<Annotations>().cloned())
    }
}

impl ToDepiction for PyProblem {
    fn to_depiction(&self, context: &DepictionContext) -> io::Result<String> {
        match &self.inner {
            Some(problem) => match problem.error_of_type::<Problems>() {
                Some(problems) => problems.to_depiction(context),
                None => problem.to_depiction(context),
            },

            None => Ok(Default::default()),
        }
    }
}

impl From<Problem> for PyProblem {
    fn from(inner: Problem) -> Self {
        Self { inner: Some(inner) }
    }
}

//
// AnnotatedPyProblem
//

struct AnnotatedPyProblem<'inner> {
    inner: &'inner PyProblem,
}

impl<'inner> ToDepiction for AnnotatedPyProblem<'inner> {
    fn to_depiction(&self, context: &DepictionContext) -> io::Result<String> {
        match &self.inner.inner {
            Some(problem) => match problem.error_of_type::<Problems>() {
                Some(problems) => problems.annotated_depiction().to_depiction(context),
                None => problem.annotated_depiction().to_depiction(context),
            },

            None => Ok(Default::default()),
        }
    }
}

impl<'inner> From<&'inner PyProblem> for AnnotatedPyProblem<'inner> {
    fn from(inner: &'inner PyProblem) -> Self {
        Self { inner }
    }
}

//
// IntoPyErr
//

/// Into Python error.
pub trait IntoPyErr {
    /// Into Python error.
    fn into_py(self) -> PyErr;
}

impl IntoPyErr for Problem {
    fn into_py(self) -> PyErr {
        PyProblem::new_err(self).unwrap_or_else(|error| error)
    }
}
