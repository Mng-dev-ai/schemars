use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use uuid::Uuid as RustUuid;

#[pyclass(subclass)]
pub struct Uuid {
    pub base: BaseField,
}

impl_py_methods!(Uuid, none, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(uuid_obj) = value.downcast::<PyAny>() {
            if uuid_obj
                .getattr("__class__")?
                .getattr("__name__")?
                .extract::<String>()?
                == "UUID"
            {
                let uuid_str = uuid_obj.call_method0("__str__")?.extract::<String>()?;
                return Ok(uuid_str.into_py(py));
            }
        }
        if !self.is_strict() {
            if let Ok(py_str) = value.downcast::<PyString>() {
                let s: &str = py_str.to_str()?;
                if let Ok(uuid) = RustUuid::parse_str(s) {
                    return Ok(uuid.to_string().into_py(py));
                }
            }
            if let Ok(py_bytes) = value.downcast::<PyBytes>() {
                let bytes = py_bytes.as_bytes();
                if bytes.len() == 16 {
                    if let Ok(uuid) = RustUuid::from_slice(bytes) {
                        return Ok(uuid.to_string().into_py(py));
                    }
                }
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Uuid", value,
        )?))
    }
});

impl_field_trait!(Uuid);
