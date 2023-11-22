use crate::errors::generate_error_msg;
use crate::errors::ValidationError;
use crate::fields::base::BaseField;
use pyo3::types::{PyDate, PyDateTime, PyFloat, PyLong, PyString};
use pyo3::{intern, prelude::*};
use speedate::Date as SpeedDate;

#[pyclass(subclass)]
pub struct Date {
    pub base: BaseField,
    format: Option<String>,
}

impl_py_methods!(Date, optional, { format: Option<String>}, {
    fn serialize(&self, py: Python, value: &PyAny) -> PyResult<PyObject> {
        if let Ok(py_datetime) = value.downcast::<PyDateTime>() {
            let py_date = py_datetime.call_method0(intern!(py, "date"))?;
            return self.format_and_return(py, py_date);
        }
        if let Ok(py_date) = value.downcast::<PyDate>() {
            return self.format_and_return(py, py_date);
        }
        if !self.is_strict() {
            if let Ok(py_str) = value.downcast::<PyString>() {
                return self.parse_and_format_date(py, py_str);
            }
            if let Ok(py_long) = value.downcast::<PyLong>() {
                return self.parse_and_format_date(py, py_long);
            }
            if let Ok(py_float) = value.downcast::<PyFloat>() {
                return self.parse_and_format_date(py, py_float);
            }
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
            "Date",
            value,
        )?))
    }

    fn parse_and_format_date(&self, py: Python, date_str: &PyAny) -> PyResult<PyObject> {
        if let Ok(date) = SpeedDate::parse_str(&date_str.to_string()) {
            let py_date = PyDate::new(py, date.year as i32, date.month, date.day)?;
            return self.format_and_return(py, py_date);
        }
        Err(PyErr::new::<ValidationError, _>(generate_error_msg(
             "Date", date_str,
        )?))
    }

    fn format_and_return(&self, py: Python, py_date: &PyAny) -> PyResult<PyObject> {
        let date_str = if let Some(ref format) = self.format {
            py_date
                .call_method1(intern!(py, "strftime"), (format,))?
                .to_string()
        } else {
            py_date.call_method0(intern!(py, "isoformat"))?.to_string()
        };
        Ok(date_str.to_object(py))
    }
});

impl_field_trait!(Date);
