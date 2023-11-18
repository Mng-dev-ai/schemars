use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(subclass)]
pub struct Dict {
    pub base: BaseField,
}

impl_py_methods!(Dict, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_dict) = value.downcast::<PyDict>() {
            return Ok(py_dict.into());
        }
        if !self.is_strict() && value.hasattr("items")? {
            let items = value.getattr("items")?.call0()?;
            let dict = PyDict::new(py);
            for item in items.iter()? {
                let (key, val): (PyObject, PyObject) = item?.extract()?;
                dict.set_item(key, val)?;
            }
            return Ok(dict.to_object(py));
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Dict", value,
        )?))
    }
});

impl_field_trait!(Dict);
