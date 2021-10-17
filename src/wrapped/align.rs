use pyo3::exceptions::PyValueError;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedAlign>()?;
    Ok(())
}

#[pyclass(name="Align", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedAlign(pub iced::Align);

#[pymethods]
impl WrappedAlign {
    #[new]
    fn new(v: &str) -> PyResult<Self> {
        Ok(Self(match v {
            "<" | "^" | "s" | "start" | "Start"  => iced::Align::Start,
            "-" | "c" | "center" | "Center" => iced::Align::Center,
            ">" | "v" | "r" | "end" | "End" => iced::Align::End,
            _ => return Err(PyValueError::new_err(v.to_owned())),
        }))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedAlign {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
