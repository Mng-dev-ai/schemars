use crate::fields::base::BaseField;
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Any {
    pub base: BaseField,
}

impl_py_methods!(Any, none, {
    fn serialize(&self, _py: Python, value: &PyAny) -> PyResult<PyObject> {
        Ok(value.into())
    }
});

impl_field_trait!(Any);
