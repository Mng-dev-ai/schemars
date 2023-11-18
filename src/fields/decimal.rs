use std::str::FromStr;

use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::PyBool;
use pyo3::types::{PyFloat, PyLong, PyString};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal as RDecimal;

#[pyclass(subclass)]
pub struct Decimal {
    pub base: BaseField,
}
impl_py_methods!(Decimal, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if value.downcast::<PyAny>()?.getattr("quantize").is_ok() {
            return Ok(value.to_string().to_object(py));
        }

        if !self.is_strict() {
            if let Ok(py_float) = value.downcast::<PyFloat>() {
                let f: f64 = py_float.extract()?;
                if let Some(decimal_value) = RDecimal::from_f64(f) {
                    return Ok(decimal_value.to_string().to_object(py));
                }
            }

            if let Ok(py_str) = value.downcast::<PyString>() {
                let s: &str = py_str.to_str()?;
                if let Ok(decimal_value) = RDecimal::from_str(s) {
                    return Ok(decimal_value.to_string().to_object(py));
                }
            }

            if let Ok(py_int) = value.downcast::<PyLong>() {
                if !value.is_instance_of::<PyBool>() {
                    let i: i64 = py_int.extract()?;
                    if let Some(decimal_value) = RDecimal::from_i64(i) {
                        return Ok(decimal_value.to_string().to_object(py));
                    }
                }
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Decimal", value,
        )?))
    }
});

impl_field_trait!(Decimal);
