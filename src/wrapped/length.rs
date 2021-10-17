use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedLength>()?;
    Ok(())
}

#[pyclass(name="Length", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedLength(pub iced::Length);

#[pymethods]
impl WrappedLength {
    #[staticmethod]
    fn fill() -> Self {
        Self(iced::Length::Fill)
    }

    #[staticmethod]
    fn fill_portion(i: u16) -> Self {
        Self(iced::Length::FillPortion(i))
    }

    #[staticmethod]
    fn shrink() -> Self {
        Self(iced::Length::Shrink)
    }

    #[staticmethod]
    fn units(i: u16) -> Self {
        Self(iced::Length::Units(i))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedLength {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
