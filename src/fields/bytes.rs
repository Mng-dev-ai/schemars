use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

#[pyclass(subclass)]
pub struct Bytes {
    pub base: BaseField,
}

impl_py_methods!(Bytes, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_bytes) = value.downcast::<PyBytes>() {
            return Ok(py_bytes.into());
        }
        if !self.is_strict() {
            if let Ok(py_str) = value.downcast::<PyString>() {
                return Ok(PyBytes::new(py, py_str.to_string().as_bytes()).into());
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Bytes", value,
        )?))
    }
});

impl_field_trait!(Bytes);
