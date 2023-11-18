use pyo3::{exceptions::PyBaseException, prelude::*};

#[pyclass(extends=PyBaseException)]
pub struct ValidationError {
    errors: PyObject,
}

#[pymethods]
impl ValidationError {
    #[new]
    pub fn new(errors: PyObject) -> Self {
        ValidationError { errors }
    }
    #[getter]
    pub fn errors(&self) -> PyObject {
        self.errors.clone()
    }
}

pub fn get_python_type(value: &PyAny) -> &'static str {
    match value {
        _ if value.is_instance_of::<pyo3::types::PyString>() => "str",
        _ if value.is_instance_of::<pyo3::types::PyBytes>() => "bytes",
        _ if value.is_instance_of::<pyo3::types::PyInt>() => "int",
        _ if value.is_instance_of::<pyo3::types::PyFloat>() => "float",
        _ if value.is_instance_of::<pyo3::types::PyBool>() => "bool",
        _ if value.is_instance_of::<pyo3::types::PyDate>() => "date",
        _ if value.is_instance_of::<pyo3::types::PyDateTime>() => "datetime",
        _ if value.is_instance_of::<pyo3::types::PyDict>() => "dict",
        _ if value.is_instance_of::<pyo3::types::PyList>() => "list",
        _ => "unknown",
    }
}

pub fn generate_error_msg(field_type: &str, value: &PyAny) -> PyResult<String> {
    let user_input_type = get_python_type(value);

    let user_input = value.to_string();
    Ok(format!(
        "Received '{}' with value '{}' which is not a valid value for '{}'",
        user_input_type, user_input, field_type
    ))
}
