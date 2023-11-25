use crate::{
    errors::ValidationError, fields::any::Any, fields::bool::Bool, fields::bytes::Bytes,
    fields::date::Date, fields::datetime::DateTime, fields::decimal::Decimal, fields::dict::Dict,
    fields::float::Float, fields::int::Int, fields::list::List, fields::method::Method,
    fields::str::Str, fields::union::Union, fields::uuid::Uuid, schema::Schema,
};
use pyo3::prelude::*;

pub trait FieldTrait {
    fn default(&self) -> Option<PyObject> {
        None
    }
    fn serialize(
        &self,
        py: Python,
        value: &PyAny,
        _parent: Option<PyObject>,
    ) -> PyResult<PyObject> {
        Ok(value.to_object(py))
    }
    fn method_getter(&self, py: Python, _field_name: &str, _parent: &PyAny) -> PyResult<PyObject> {
        Ok(py.None())
    }
    fn is_write_only(&self) -> bool {
        false
    }
    fn source(&self) -> Option<String> {
        None
    }
    fn call(&self) -> bool {
        false
    }
    fn is_method_field(&self) -> bool {
        false
    }
    fn alias(&self) -> Option<String> {
        None
    }
}

#[derive(Clone)]
pub enum Field {
    Str(Py<Str>),
    Bytes(Py<Bytes>),
    Int(Py<Int>),
    Bool(Py<Bool>),
    Float(Py<Float>),
    Decimal(Py<Decimal>),
    Date(Py<Date>),
    DateTime(Py<DateTime>),
    Dict(Py<Dict>),
    List(Py<List>),
    Uuid(Py<Uuid>),
    Union(Py<Union>),
    Any(Py<Any>),
    Method(Py<Method>),
    NestedSchema(Py<Schema>, bool),
}

impl Field {
    fn with_field_ref<R, F: FnOnce(&dyn FieldTrait) -> PyResult<R>>(
        &self,
        py: Python,
        f: F,
    ) -> PyResult<R> {
        match self {
            Field::Str(field) => f(&*field.as_ref(py).borrow()),
            Field::Bytes(field) => f(&*field.as_ref(py).borrow()),
            Field::Int(field) => f(&*field.as_ref(py).borrow()),
            Field::Bool(field) => f(&*field.as_ref(py).borrow()),
            Field::Float(field) => f(&*field.as_ref(py).borrow()),
            Field::Decimal(field) => f(&*field.as_ref(py).borrow()),
            Field::Date(field) => f(&*field.as_ref(py).borrow()),
            Field::DateTime(field) => f(&*field.as_ref(py).borrow()),
            Field::Dict(field) => f(&*field.as_ref(py).borrow()),
            Field::List(field) => f(&*field.as_ref(py).borrow()),
            Field::Uuid(field) => f(&*field.as_ref(py).borrow()),
            Field::Union(field) => f(&*field.as_ref(py).borrow()),
            Field::Any(field) => f(&*field.as_ref(py).borrow()),
            Field::Method(field) => f(&*field.as_ref(py).borrow()),
            Field::NestedSchema(field, _) => f(&*field.as_ref(py).borrow()),
        }
    }
    pub fn default_value(&self, py: Python) -> PyResult<Option<PyObject>> {
        self.with_field_ref(py, |field: &dyn FieldTrait| Ok(field.default()))
    }
    pub fn serialize(
        &self,
        py: Python,
        value: &PyAny,
        parent: Option<PyObject>,
    ) -> PyResult<PyObject> {
        match self {
            // If the field is schema, we will ignore the trait and use the schema's serialize method directly.
            // This is because we need to pass the many flag to the schema's serialize method.
            Field::NestedSchema(field, many) => {
                let field_ref = field.as_ref(py).borrow();
                if *many {
                    field_ref.serialize(py, value, Some(true), parent)
                } else {
                    field_ref.serialize(py, value, None, parent)
                }
            }
            _ => self.with_field_ref(py, |field| field.serialize(py, value, parent)),
        }
    }
    pub fn method_getter(
        &self,
        py: Python,
        field_name: &str,
        parent: &PyAny,
    ) -> PyResult<PyObject> {
        self.with_field_ref(py, |field| field.method_getter(py, field_name, parent))
    }
    pub fn is_write_only(&self, py: Python) -> PyResult<bool> {
        self.with_field_ref(py, |field| Ok(field.is_write_only()))
    }
    pub fn source(&self, py: Python) -> PyResult<Option<String>> {
        self.with_field_ref(py, |field| Ok(field.source()))
    }
    pub fn is_method_field(&self, py: Python) -> PyResult<bool> {
        self.with_field_ref(py, |field| Ok(field.is_method_field()))
    }
    pub fn call(&self, py: Python) -> PyResult<bool> {
        self.with_field_ref(py, |field| Ok(field.call()))
    }
    pub fn alias(&self, py: Python) -> PyResult<Option<String>> {
        self.with_field_ref(py, |field| Ok(field.alias()))
    }
}

impl<'source> FromPyObject<'source> for Field {
    #[inline]
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        if let Ok(field) = obj.extract::<Py<Str>>() {
            Ok(Field::Str(field))
        } else if let Ok(field) = obj.extract::<Py<Bytes>>() {
            Ok(Field::Bytes(field))
        } else if let Ok(field) = obj.extract::<Py<Int>>() {
            Ok(Field::Int(field))
        } else if let Ok(field) = obj.extract::<Py<Bool>>() {
            Ok(Field::Bool(field))
        } else if let Ok(field) = obj.extract::<Py<Float>>() {
            Ok(Field::Float(field))
        } else if let Ok(field) = obj.extract::<Py<Decimal>>() {
            Ok(Field::Decimal(field))
        } else if let Ok(field) = obj.extract::<Py<Date>>() {
            Ok(Field::Date(field))
        } else if let Ok(field) = obj.extract::<Py<DateTime>>() {
            Ok(Field::DateTime(field))
        } else if let Ok(field) = obj.extract::<Py<Dict>>() {
            Ok(Field::Dict(field))
        } else if let Ok(field) = obj.extract::<Py<List>>() {
            Ok(Field::List(field))
        } else if let Ok(field) = obj.extract::<Py<Uuid>>() {
            Ok(Field::Uuid(field))
        } else if let Ok(field) = obj.extract::<Py<Union>>() {
            Ok(Field::Union(field))
        } else if let Ok(field) = obj.extract::<Py<Any>>() {
            Ok(Field::Any(field))
        } else if let Ok(field) = obj.extract::<Py<Method>>() {
            Ok(Field::Method(field))
        } else if let Ok((field, many)) = obj.extract::<(Py<Schema>, bool)>() {
            Ok(Field::NestedSchema(field, many))
        } else {
            Err(PyErr::new::<ValidationError, _>(["Invalid field type."]))
        }
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
pub struct BaseField {
    #[pyo3(get, set)]
    pub write_only: bool,
    #[pyo3(get, set)]
    pub strict: bool,
    #[pyo3(get, set)]
    pub call: bool,
    #[pyo3(get, set)]
    pub default: Option<PyObject>,
    #[pyo3(get, set)]
    pub source: Option<String>,
    #[pyo3(get, set)]
    pub serialize_func: Option<PyObject>,
    #[pyo3(get, set)]
    pub alias: Option<String>,
    pub is_method_field: bool,
}

impl BaseField {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        write_only: bool,
        strict: bool,
        call: bool,
        default: Option<PyObject>,
        source: Option<String>,
        serialize_func: Option<PyObject>,
        alias: Option<String>,
        is_method_field: bool,
    ) -> Self {
        BaseField {
            write_only,
            strict,
            call,
            default,
            source,
            serialize_func,
            alias,
            is_method_field,
        }
    }
}
