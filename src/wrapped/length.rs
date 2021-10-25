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

/// The strategy used to fill space in a specific dimension.
///
/// .. seealso::
///     * `iced::Length <https://docs.rs/iced/0.3.0/iced/enum.Length.html>`_
#[pyclass(name = "Length", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedLength(pub Length, Private);

#[pymethods]
impl WrappedLength {
    /// Fill a portion of the remaining space relative to other elements.
    #[staticmethod]
    fn fill_portion(i: u16) -> Self {
        Self(Length::FillPortion(i), Private)
    }

    /// Fill a fixed amount of space.
    #[staticmethod]
    fn units(i: u16) -> Self {
        Self(Length::Units(i), Private)
    }

    /// Fill all the remaining space.
    #[classattr]
    #[allow(non_snake_case)]
    fn FILL() -> Self {
        Self(Length::Fill, Private)
    }

    /// Fill the least amount of space.
    #[classattr]
    #[allow(non_snake_case)]
    fn SHRINK() -> Self {
        Self(Length::Shrink, Private)
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedLength {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
