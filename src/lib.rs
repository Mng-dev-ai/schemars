#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
mod macros;

mod errors;
mod fields;
mod schema;

use pyo3::prelude::*;

#[pymodule]
fn _schemars(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<schema::Schema>()?;
    m.add_class::<fields::str::Str>()?;
    m.add_class::<fields::bytes::Bytes>()?;
    m.add_class::<fields::int::Int>()?;
    m.add_class::<fields::bool::Bool>()?;
    m.add_class::<fields::float::Float>()?;
    m.add_class::<fields::date::Date>()?;
    m.add_class::<fields::datetime::DateTime>()?;
    m.add_class::<fields::dict::Dict>()?;
    m.add_class::<fields::list::List>()?;
    m.add_class::<fields::uuid::Uuid>()?;
    m.add_class::<fields::union::Union>()?;
    m.add_class::<fields::any::Any>()?;
    m.add_class::<fields::method::Method>()?;
    m.add_class::<fields::decimal::Decimal>()?;
    m.add_class::<errors::ValidationError>()?;
    Ok(())
}
