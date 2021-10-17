pub(crate) mod elements;
pub(crate) mod wrapped;
pub(crate) mod to_native;
pub(crate) mod app;
pub(crate) mod wrapped_states;

use std::fmt::{Debug, Write};

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pymodule]
fn pyiced(py: Python, m: &PyModule) -> PyResult<()> {
    app::init(py, m)?;
    wrapped::init(py, m)?;
    wrapped_states::init(py, m)?;
    Ok(())
}

pub(crate) fn debug_str(value: &dyn Debug) -> PyResult<String> {
    let mut result = String::new();
    let err = match write!(&mut result, "{:#?}", value) {
        Ok(()) => return Ok(result),
        Err(err) => err,
    };

    let mut result = String::new();
    match write!(&mut result, "{:#?}", err) {
        Ok(()) => return Err(PyException::new_err(result)),
        Err(err) => {
            dbg!(err);
            Err(PyException::new_err("<EXCEPTION>"))
        },
    }
}
