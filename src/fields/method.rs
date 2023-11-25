use crate::fields::base::{BaseField, FieldTrait};
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Method {
    pub base: BaseField,
    pub method_name: Option<String>,
}

#[pymethods]
impl Method {
    #[new]
    #[pyo3(signature=(method_name=None))]
    fn new(method_name: Option<String>) -> Self {
        Method {
            base: BaseField::new(false, false, false, None, None, None, None, true),
            method_name,
        }
    }
}

impl FieldTrait for Method {
    fn method_getter(&self, py: Python, field_name: &str, parent: &PyAny) -> PyResult<PyObject> {
        let method_name: String = self
            .method_name
            .clone()
            .unwrap_or_else(|| format!("get_{}", field_name));
        let method: Result<Py<PyAny>, PyErr> = parent.into_py(py).getattr(py, method_name.as_str());
        Ok(method?.to_object(py))
    }
    fn is_method_field(&self) -> bool {
        self.base.is_method_field
    }
}
