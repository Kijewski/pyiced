use std::borrow::Cow;

use iced::Length;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_cow;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedLength>()?;
    Ok(())
}

/// The strategy used to fill space in a specific dimension.
///
/// See also
/// --------
/// `iced::Length <https://docs.rs/iced/0.3.0/iced/enum.Length.html>`_
#[pyclass(name = "Length", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedLength(pub Length);

#[pymethods]
impl WrappedLength {
    /// fill_portion(i)
    /// --
    ///
    /// Fill a portion of the remaining space relative to other elements.
    #[staticmethod]
    fn fill_portion(i: u16) -> Self {
        Self(Length::FillPortion(i))
    }

    /// units(i)
    /// --
    ///
    /// Fill a fixed amount of space.
    #[staticmethod]
    fn units(i: u16) -> Self {
        Self(Length::Units(i))
    }

    /// Fill all the remaining space.
    #[classattr]
    #[allow(non_snake_case)]
    fn FILL() -> Self {
        Self(Length::Fill)
    }

    /// Fill the least amount of space.
    #[classattr]
    #[allow(non_snake_case)]
    fn SHRINK() -> Self {
        Self(Length::Shrink)
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<Cow<'static, str>> {
        match self.0 {
            Length::Fill => Ok(Cow::Borrowed("Length.FILL")),
            Length::Shrink => Ok(Cow::Borrowed("Length.SHRINK")),
            Length::FillPortion(i) => format_to_cow!("Length.fill_portion({:?})", i),
            Length::Units(i) => format_to_cow!("Length.units({:?})", i),
        }
    }
}
