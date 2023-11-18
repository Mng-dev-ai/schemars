use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::types::PyDateAccess;
use pyo3::types::{PyDate, PyDateTime, PyFloat, PyLong, PyString};
use pyo3::{intern, prelude::*};
use speedate::DateTime as SpeedDateTime;

#[pyclass(subclass)]
pub struct DateTime {
    pub base: BaseField,
    format: Option<String>,
}
impl_py_methods!(DateTime, optional, { format: Option<String>}, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_datetime) = value.downcast::<PyDateTime>() {
            return self.format_and_return(py, py_datetime);
        }
        if !self.is_strict() {
            if let Ok(py_date) = value.downcast::<PyDate>() {
                let py_datetime = PyDateTime::new(
                    py,
                    py_date.get_year(),
                    py_date.get_month(),
                    py_date.get_day(),
                    0,
                    0,
                    0,
                    0,
                    None, // Default time to midnight
                )?;
                return self.format_and_return(py, py_datetime);
            }
            if let Ok(py_str) = value.downcast::<PyString>() {
                return self.parse_and_format_datetime(py, py_str);
            }
            if let Ok(py_long) = value.downcast::<PyLong>() {
                return self.parse_and_format_datetime(py, py_long);
            }
            if let Ok(py_float) = value.downcast::<PyFloat>() {
                return self.parse_and_format_datetime(py, py_float);
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "DateTime",
            value,
        )?))
    }

    fn parse_and_format_datetime(&self, py: Python, datetime_str: &PyAny) -> PyResult<PyObject> {
        if let Ok(datetime) = SpeedDateTime::parse_str(&datetime_str.to_string()) {
            let py_datetime = PyDateTime::new(
                py,
                datetime.date.year as i32,
                datetime.date.month,
                datetime.date.day,
                datetime.time.hour,
                datetime.time.minute,
                datetime.time.second,
                datetime.time.microsecond,
                None,
            )?;
            return self.format_and_return(py, py_datetime);
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "DateTime",
            datetime_str,
        )?))
    }

    fn format_and_return(&self, py: Python, py_datetime: &PyAny) -> PyResult<PyObject> {
        let datetime_str = if let Some(ref format) = self.format {
            py_datetime
                .call_method1(intern!(py, "strftime"), (format,))?
                .to_string()
        } else {
            py_datetime
                .call_method0(intern!(py, "isoformat"))?
                .to_string()
        };
        Ok(datetime_str.to_object(py))
    }
});

impl_field_trait!(DateTime);
