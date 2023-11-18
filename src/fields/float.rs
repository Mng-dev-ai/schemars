use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyLong, PyString};

#[pyclass(subclass)]
pub struct Float {
    pub base: BaseField,
}

impl_py_methods!(Float, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_float) = value.downcast::<PyFloat>() {
            return Ok(py_float.into());
        }
        if !self.is_strict() {
            if let Ok(py_bool) = value.downcast::<PyBool>() {
                return Ok((py_bool.is_true() as i32).to_object(py));
            }
            if let Ok(py_str) = value.downcast::<PyString>() {
                let s: &str = py_str.to_str()?;
                if let Ok(f) = s.parse::<f64>() {
                    return Ok(f.to_object(py));
                }
            }
            if let Ok(py_int) = value.downcast::<PyLong>() {
                let i: i64 = py_int.extract()?;
                return Ok((i as f64).to_object(py));
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Float", value,
        )?))
    }
});

impl_field_trait!(Float);
