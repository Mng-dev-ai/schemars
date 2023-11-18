use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyLong, PyString};

#[pyclass(subclass)]
pub struct Int {
    pub base: BaseField,
}

impl_py_methods!(Int, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_int) = value.downcast::<PyLong>() {
            return Ok(py_int.into());
        }
        if !self.is_strict() {
            if let Ok(py_bool) = value.downcast::<PyBool>() {
                return Ok((py_bool.is_true() as i32).to_object(py));
            }
            if let Ok(py_str) = value.downcast::<PyString>() {
                let s: &str = py_str.to_str()?;
                if let Ok(i) = s.parse::<i32>() {
                    return Ok(i.to_object(py));
                }
            }
            if let Ok(py_float) = value.downcast::<PyFloat>() {
                let f: f64 = py_float.extract()?;
                if f.fract() == 0.0 && f >= i32::MIN as f64 && f <= i32::MAX as f64 {
                    return Ok((f as i32).to_object(py));
                }
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Int", value,
        )?))
    }
});

impl_field_trait!(Int);
