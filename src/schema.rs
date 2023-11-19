use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use crate::fields::base::Field;
use crate::fields::base::FieldTrait;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct Schema {
    fields: HashMap<String, Field>,
    pub base: BaseField,
    #[pyo3(get, set)]
    context: HashMap<String, PyObject>,
}
impl_py_methods!(Schema, required, { fields: HashMap<String, Field>, context: HashMap<String, PyObject>}, {
    fn get_attr_value(
        &self,
        py: Python,
        mut instance: PyObject,
        field: Field,
        key: &str,
    ) -> PyResult<PyObject> {
        let attr = if let Some(source) = field.source(py)? {
            for attr_name in source.split('.') {
                instance = instance.getattr(py, attr_name)?;
            }
            instance
        } else {
            instance.getattr(py, key)?
        };
        if field.call(py)? {
            attr.call0(py)
        } else {
            Ok(attr)
        }
    }
    fn serialize_attr_value(
        &self,
        py: Python,
        attr_value: PyObject,
        field: Field,
    ) -> PyResult<PyObject> {
        if attr_value.is_none(py) {
            field.default_value(py)?.map_or_else(
                || Ok(py.None()),
                |default_value| Ok(default_value.to_object(py)),
            )
        } else {
            field.serialize(py, attr_value.as_ref(py), None)
        }
    }
    fn handle_method_field(
        &self,
        py: Python,
        key: &str,
        field: Field,
        instance: PyObject,
        parent: Option<PyObject>,
    ) -> PyResult<PyObject> {
        let method_result = field.method_getter(py, key, parent.clone().into_py(py).as_ref(py))?;
        let result = method_result.call1(py, (self.clone(),instance,))?;
        Ok(result)
    }

    fn add_error(&self, py: Python, errors: &PyDict, key: &str, error: PyObject) -> PyResult<()> {
        errors.set_item(key, error.call_method0(py, intern!(py, "__str__"))?)
    }

    pub fn serialize(
        &self,
        py: Python,
        instance: &PyAny,
        many: Option<bool>,
        parent: Option<PyObject>,
    ) -> PyResult<PyObject> {
        if let Some(callback) = &self.base.serialize_func {
            let result = callback.call1(py, (instance,))?;
            return Ok(result);
        }
        if let Some(true) = many {
            let pylist = instance.downcast::<PyList>()?;
            let mut results: Vec<PyObject> = Vec::with_capacity(pylist.len());
            for inst in pylist.iter() {
                let serialized = self.serialize_one(py, inst, parent.clone())?;
                results.push(serialized);
            }
            return Ok(results.into_py(py));
        }
        self.serialize_one(py, instance, parent.clone())
    }
    fn serialize_one(
        &self,
        py: Python,
        instance: &PyAny,
        parent: Option<PyObject>,
    ) -> PyResult<PyObject> {
        let serialized_data = PyDict::new(py);
        let errors = PyDict::new(py);
        for (key, field) in &self.fields {
            if field.is_write_only(py)? {
                continue;
            }
            if field.is_method_field(py)? {
                match self.handle_method_field(
                    py,
                    key,
                    field.clone(),
                    instance.into(),
                    parent.clone(),
                ) {
                    Ok(value) => serialized_data.set_item(key, value)?,
                    Err(e) => {
                        self.add_error(py, errors, key, e.to_object(py))?;
                    }
                }
                continue;
            }
            let attr_value = self
                .get_attr_value(py, instance.into(), field.clone(), key)
                .and_then(|val| self.serialize_attr_value(py, val, field.clone()))
                .or_else(|e| {
                    self.add_error(py, errors, key, e.to_object(py))?;
                    Err(e)
                });

            if let Ok(value) = attr_value {
                serialized_data.set_item(key, value)?;
            }
        }

        if !errors.is_empty() {
            Err(PyErr::new::<ValidationError, _>(errors.to_object(py)))
        } else {
            Ok(serialized_data.into())
        }
    }

});

impl FieldTrait for Schema {
    fn default(&self) -> Option<PyObject> {
        self.base.default.clone()
    }
    fn is_write_only(&self) -> bool {
        self.base.write_only
    }
    fn source(&self) -> Option<String> {
        self.base.source.clone()
    }
    fn is_method_field(&self) -> bool {
        self.base.is_method_field
    }
    fn call(&self) -> bool {
        self.base.call
    }
}
