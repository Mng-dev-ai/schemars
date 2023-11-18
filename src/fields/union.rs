use crate::errors::{generate_error_msg, ValidationError};
use crate::fields::base::BaseField;
use pyo3::intern;
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Union {
    pub base: BaseField,
    fields: Vec<PyObject>,
}

impl_py_methods!(Union, required, { fields: Vec<PyObject> }, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        for field in &self.fields {
            if let Ok(py_field) = field.downcast::<PyAny>(py) {
                if let Ok(py_value) =
                    py_field.call_method1(intern!(py, "serialize"), (value,))
                {
                    return Ok(py_value.to_object(py));
                }
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Union", value,
       )?))
   }
});

impl_field_trait!(Union);
