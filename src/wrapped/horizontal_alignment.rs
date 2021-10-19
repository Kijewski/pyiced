use iced::HorizontalAlignment;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedHorizontalAlignment>()?;
    Ok(())
}

#[pyclass(name = "HorizontalAlignment", module = "pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedHorizontalAlignment(pub HorizontalAlignment);

#[pymethods]
impl WrappedHorizontalAlignment {
    #[new]
    fn new(v: &str) -> PyResult<Self> {
        Ok(Self(match v {
            "<" | "l" | "left" | "Left" => HorizontalAlignment::Left,
            "-" | "c" | "center" | "Center" => HorizontalAlignment::Center,
            ">" | "r" | "right" | "Right" => HorizontalAlignment::Right,
            _ => return Err(PyValueError::new_err(v.to_owned())),
        }))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedHorizontalAlignment {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
