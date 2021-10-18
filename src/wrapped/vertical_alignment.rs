use iced::VerticalAlignment;
use pyo3::exceptions::PyValueError;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedVerticalAlignment>()?;
    Ok(())
}

#[pyclass(name="VerticalAlignment", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedVerticalAlignment(pub VerticalAlignment);

#[pymethods]
impl WrappedVerticalAlignment {
    #[new]
    fn new(v: &str) -> PyResult<Self> {
        Ok(Self(match v {
            "^" | "t" | "top" | "Top" => VerticalAlignment::Top,
            "-" | "c" | "center" | "Center" => VerticalAlignment::Center,
            "v" | "b" | "bottom" | "Bottom" => VerticalAlignment::Bottom,
            _ => return Err(PyValueError::new_err(v.to_owned())),
        }))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedVerticalAlignment {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
