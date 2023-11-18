use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass(subclass)]
pub struct List {
    pub base: BaseField,
    child: Option<PyObject>,
}

impl_py_methods!(List, optional, { child: Option<PyObject> }, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        let as_list = |v: &PyAny| -> PyResult<_> {
            match v.downcast::<PyList>() {
                Ok(py_list) => Ok(py_list.into()),
                _ if !self.is_strict() => {
                    let temp_list = PyList::empty(py);
                    for item in v.iter()? {
                        temp_list.append(item?)?;
                    }
                    Ok(temp_list.to_object(py))
                }
                _ => Err(PyErr::new::<ValidationError, _>(generate_error_msg(
                    "List",
                    value,
                )?)),
            }
        };
        match self.child {
            Some(ref child) => {
                let list = as_list(value)?;
                let downcasted_list = PyList::empty(py);
                for item in list.as_ref(py).iter()? {
                    let py_item = child
                        .as_ref(py)
                        .call_method1(intern!(py, "serialize"), (item?,))?;
                    downcasted_list.append(py_item)?;
                }
                Ok(downcasted_list.to_object(py))
            }
            None => as_list(value),
        }
    }
});

impl_field_trait!(List);
