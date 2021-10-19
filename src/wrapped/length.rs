use iced::Length;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedLength>()?;
    Ok(())
}

#[pyclass(name = "Length", module = "pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedLength(pub Length);

#[pymethods]
impl WrappedLength {
    #[staticmethod]
    fn fill() -> Self {
        Self(Length::Fill)
    }

    #[staticmethod]
    fn fill_portion(i: u16) -> Self {
        Self(Length::FillPortion(i))
    }

    #[staticmethod]
    fn shrink() -> Self {
        Self(Length::Shrink)
    }

    #[staticmethod]
    fn units(i: u16) -> Self {
        Self(Length::Units(i))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedLength {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
