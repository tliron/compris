use super::super::super::annotate::*;

use {depiction::*, pyo3::prelude::*, std::io};

//
// PyLocation
//

/// Location.
#[pyclass(name = "Location", frozen)]
pub struct PyLocation {
    /// Inner location.
    pub inner: Location,
}

#[pymethods]
impl PyLocation {
    /// Index.
    #[getter]
    pub fn index(&self) -> Option<usize> {
        self.inner.index
    }

    /// Row.
    #[getter]
    pub fn row(&self) -> Option<usize> {
        self.inner.row
    }

    /// Column.
    #[getter]
    pub fn column(&self) -> Option<usize> {
        self.inner.column
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

impl ToDepiction for PyLocation {
    fn to_depiction(&self, context: &DepictionContext) -> io::Result<String> {
        self.inner.to_depiction(context)
    }
}

impl From<Location> for PyLocation {
    fn from(inner: Location) -> Self {
        Self { inner }
    }
}
