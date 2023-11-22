use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyLong, PyString};

const TRUTHY: [&str; 6] = ["t", "true", "on", "y", "yes", "1"];
const FALSY: [&str; 6] = ["f", "false", "off", "n", "no", "0"];

#[pyclass(subclass)]
pub struct Bool {
    pub base: BaseField,
}

impl_py_methods!(Bool, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_bool) = value.downcast::<PyBool>() {
            return Ok(py_bool.into());
        }
        if !self.is_strict() {
            if let Ok(py_str) = value.downcast::<PyString>() {
                let s: &str = py_str.to_str()?;
                if TRUTHY.contains(&s) {
                    return Ok(PyBool::new(py, true).into());
                }
                if FALSY.contains(&s) {
                    return Ok(PyBool::new(py, false).into());
                }
            }
            if let Ok(py_long) = value.downcast::<PyLong>() {
                let i: isize = py_long.extract()?;
                if i == 1 {
                    return Ok(PyBool::new(py, true).into());
                }
                if i == 0 {
                    return Ok(PyBool::new(py, false).into());
                }
            }
            if let Ok(py_float) = value.downcast::<PyFloat>() {
                let f: f64 = py_float.extract()?;
                if f == 1.0 {
                    return Ok(PyBool::new(py, true).into());
                }
                if f == 0.0 {
                    return Ok(PyBool::new(py, false).into());
                }
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Bool", value,
        )?))
    }
});

impl_field_trait!(Bool);
