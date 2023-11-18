use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use std::borrow::Cow;

#[pyclass(subclass)]
pub struct Str {
    pub base: BaseField,
}

impl_py_methods!(Str, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_str) = value.downcast::<PyString>() {
            return Ok(py_str.into());
        }
        if !self.is_strict() {
            if let Ok(py_bytes) = value.downcast::<PyBytes>() {
                if let Ok(utf8_str) = std::str::from_utf8(py_bytes.as_bytes()) {
                    return Ok(utf8_str.to_object(py));
                }
                let cow: Cow<str> = String::from_utf8_lossy(py_bytes.as_bytes());
                return Ok(cow.into_owned().to_object(py));
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Str", value,
        )?))
    }
});

impl_field_trait!(Str);
