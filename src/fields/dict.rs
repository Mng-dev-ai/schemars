use crate::errors::{generate_error_msg, ValidationError};
use crate::fields::base::{BaseField, Field};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(subclass)]
pub struct Dict {
    pub base: BaseField,
    child: Option<Field>,
}

impl_py_methods!(Dict, optional, { child: Option<Field>}, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_dict) = value.downcast::<PyDict>() {
            let dict = PyDict::new(py);
            for (key, val) in py_dict.iter() {
                let value = self.serialize_child(py, val)?;
                dict.set_item(key, value)?;
            }
            return Ok(dict.into());
        }
        if !self.is_strict() && value.hasattr("items")? {
            let items = value.getattr("items")?.call0()?;
            let dict = PyDict::new(py);
            for item in items.iter()? {
                let (key, val): (PyObject, PyObject) = item?.extract()?;
                let value = self.serialize_child(py, val.as_ref(py))?;
                dict.set_item(key, value)?;
            }
            return Ok(dict.into());
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg("Dict", value)?))
    }
    fn serialize_child(&self, py: Python, child: &PyAny) -> PyResult<PyObject> {
        match &self.child {
            Some(child_type) => child_type.serialize(py, child, None),
            None => Ok(child.into()),
        }
    }

});

impl_field_trait!(Dict);
