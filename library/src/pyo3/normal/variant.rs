use super::super::super::{annotate::*, normal::*};

use pyo3::{exceptions::*, prelude::*, types::*};

//
// ToPy
//

/// To Python.
pub trait ToPy {
    /// To Python.
    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>;
}

impl ToPy for Variant<WithoutAnnotations> {
    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(match self {
            Variant::Undefined => PyNotImplemented::get(py).to_owned().into_any(),
            Variant::Null(_) => PyNone::get(py).to_owned().into_any(),
            Variant::Integer(integer) => PyInt::new(py, integer.inner).into_any(),
            Variant::UnsignedInteger(unsigned_integer) => PyInt::new(py, unsigned_integer.inner).into_any(),
            Variant::Float(float) => PyFloat::new(py, float.into()).into_any(),
            Variant::Boolean(boolean) => PyBool::new(py, boolean.into()).to_owned().into_any(),
            Variant::Text(text) => PyString::new(py, text.as_ref()).into_any(),
            Variant::Blob(blob) => PyBytes::new(py, blob.as_ref()).into_any(),

            Variant::List(list) => {
                let mut items = Vec::with_capacity(list.inner.len());
                for item in list {
                    items.push(item.to_py(py)?);
                }
                PyList::new(py, items)?.into_any()
            }

            Variant::Map(map) => {
                let dict = PyDict::new(py);
                for (key, value) in map {
                    let is_collection = key.is_collection();

                    let py_key = key.to_py(py)?;
                    let py_key = if is_collection { py_key.repr()?.into_any() } else { py_key };
                    let py_value = value.to_py(py)?;

                    dict.set_item(py_key, py_value)?;
                }
                dict.into_any()
            }
        })
    }
}

//
// FromPy
//

/// From Python.
pub trait FromPy {
    /// From Python.
    fn from_py(&self) -> PyResult<Variant<WithoutAnnotations>>;
}

impl<'py> FromPy for Bound<'py, PyAny> {
    fn from_py(&self) -> PyResult<Variant<WithoutAnnotations>> {
        if self.is_none() {
            Ok(Null::default().into())
        } else if let Ok(float) = self.cast::<PyFloat>() {
            Ok(Float::from(float.value()).into())
        } else if let Ok(boolean) = self.cast::<PyBool>() {
            Ok(Boolean::from(boolean.is_true()).into())
        } else if let Ok(mapping) = self.cast::<PyMapping>() {
            let mut map = Map::default();

            for item in mapping.items()? {
                let key = item.get_item(0)?.from_py()?;
                let value = item.get_item(1)?.from_py()?;
                map.into_insert(key, value);
            }

            Ok(map.into())
        } else if let Ok(sequence) = self.cast::<PySequence>() {
            let len = sequence.len()?;
            let mut list = List::new_with_capacity(len);

            for index in 0..len {
                let value = sequence.get_item(index)?.from_py()?;
                list.into_push(value);
            }

            Ok(list.into())
        } else if let Ok(unsigned_integer) = self.extract::<u64>() {
            Ok(UnsignedInteger::from(unsigned_integer).into())
        } else if let Ok(integer) = self.extract::<i64>() {
            Ok(Integer::from(integer).into())
        } else if let Ok(string) = self.extract::<String>() {
            Ok(Text::from(string).into())
        } else if let Ok(bytes) = self.extract::<Vec<u8>>() {
            Ok(Blob::from(bytes).into())
        } else {
            Err(PyTypeError::new_err("unsupported type"))
        }
    }
}
