#[macro_export]
macro_rules! impl_field_trait {
    ($type:ty) => {
        use $crate::fields::base::FieldTrait;
        impl FieldTrait for $type {
            fn default(&self) -> Option<PyObject> {
                self.base.default.clone()
            }
            fn serialize(
                &self,
                py: Python,
                value: &PyAny,
                _parent: Option<PyObject>,
            ) -> PyResult<PyObject> {
                if let Some(callback) = &self.base.serialize_func {
                    let result = callback.call1(py, (value,))?;
                    return Ok(result);
                }

                return self.serialize(py, value);
            }
            fn is_write_only(&self) -> bool {
                self.base.write_only
            }
            fn source(&self) -> Option<String> {
                self.base.source.clone()
            }
            fn call(&self) -> bool {
                self.base.call
            }
            fn is_method_field(&self) -> bool {
                self.base.is_method_field
            }
            fn alias(&self) -> Option<String> {
                self.base.alias.clone()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_py_methods {
    ($struct_name:ident, none, { $($method:item)* }) => {
        #[pymethods]
        impl $struct_name {
            #[allow(clippy::too_many_arguments)]
            #[new]
            #[pyo3(signature=(write_only=false, strict=false, call=false, default=None, source=None, serialize_func=None, alias=None))]
            fn new(
                write_only: bool,
                strict: bool,
                call: bool,
                default: Option<PyObject>,
                source: Option<String>,
                serialize_func: Option<PyObject>,
                alias: Option<String>,
            ) -> Self {
                $struct_name {
                    base: BaseField::new(write_only, strict, call, default, source, serialize_func, alias, false),
                }
            }

            #[getter]
            fn is_strict(&self) -> bool {
                self.base.strict
            }

            $($method)*
        }
    };

    ($struct_name:ident, optional, { $($field_name:ident: Option<$field_type:ty>),* $(,)? }, { $($method:item)* }) => {
        #[pymethods]
        impl $struct_name {
            #[allow(clippy::too_many_arguments)]
            #[new]
            #[pyo3(signature=($($field_name=None)*, write_only=false, strict=false, call=false, default=None, source=None, serialize_func=None, alias=None))]
            fn new(
                $( $field_name: Option<$field_type>, )*
                write_only: bool,
                strict: bool,
                call: bool,
                default: Option<PyObject>,
                source: Option<String>,
                serialize_func: Option<PyObject>,
                alias: Option<String>,

            ) -> Self {
                $struct_name {
                    base: BaseField::new(write_only, strict, call, default, source, serialize_func, alias, false),
                    $( $field_name, )*
                }
            }

            #[getter]
            fn is_strict(&self) -> bool {
                self.base.strict
            }

            $($method)*
        }
    };

    ($struct_name:ident, required, { $($field_name:ident: $field_type:ty),* $(,)? }, { $($method:item)* }) => {
        #[pymethods]
        impl $struct_name {
            #[allow(clippy::too_many_arguments)]
            #[new]
            #[pyo3(signature=( $($field_name),*, write_only=false, strict=false, call=false, default=None, source= None, serialize_func= None, alias= None))]
            fn new(
                $( $field_name: $field_type, )*
                write_only: bool,
                strict: bool,
                call: bool,
                default: Option<PyObject>,
                source: Option<String>,
                serialize_func: Option<PyObject>,
                alias: Option<String>,
            ) -> Self {
                $struct_name {
                    base: BaseField::new(write_only, strict, call, default, source, serialize_func, alias, false),
                    $( $field_name, )*
                }
            }

            #[getter]
            fn is_strict(&self) -> bool {
                self.base.strict
            }

            $($method)*
        }
    };
}
