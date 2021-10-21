use iced::Length;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedLength>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

#[pyclass(name = "Length", module = "pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedLength(pub Length, Private);

#[pymethods]
impl WrappedLength {
    #[staticmethod]
    fn fill() -> Self {
        Self(Length::Fill, Private)
    }

    #[staticmethod]
    fn fill_portion(i: u16) -> Self {
        Self(Length::FillPortion(i), Private)
    }

    #[staticmethod]
    fn shrink() -> Self {
        Self(Length::Shrink, Private)
    }

    #[staticmethod]
    fn units(i: u16) -> Self {
        Self(Length::Units(i), Private)
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedLength {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
