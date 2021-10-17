use iced::Size;
use pyo3::prelude::*;

use crate::common::{debug_str, f32_nonneg};
use crate::format_to_py;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSize>()?;
    Ok(())
}

/// Size(width, height)
/// --
///
/// An amount of space in 2 dimensions.
///
/// Parameters
/// ----------
/// width : float
///     The width.
/// height : float
///     The height.
///
/// See also
/// --------
/// `iced::Size <https://docs.rs/iced/0.3.0/iced/struct.Size.html>`_
#[pyclass(name = "Size", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSize(pub Size);

#[pymethods]
impl WrappedSize {
    #[new]
    fn new(width: f32, height: f32) -> PyResult<Self> {
        Ok(Self(Size {
            width: f32_nonneg(width)?,
            height: f32_nonneg(height)?,
        }))
    }

    /// The width.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "width" parameter given when constructing this size.
    #[getter]
    fn width(&self) -> f32 {
        self.0.width
    }

    /// The height.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "height" parameter given when constructing this size.
    #[getter]
    fn height(&self) -> f32 {
        self.0.height
    }

    /// A Size with zero width and height.
    #[classattr]
    #[allow(non_snake_case)]
    fn ZERO() -> Self {
        Self(Size::ZERO)
    }

    /// A Size with a width and height of 1 unit.
    #[classattr]
    #[allow(non_snake_case)]
    fn UNIT() -> Self {
        Self(Size::UNIT)
    }

    /// A Size with infinite width and height.
    #[classattr]
    #[allow(non_snake_case)]
    fn INFINITY() -> Self {
        Self(Size::INFINITY)
    }

    /// pad($self, /, padding)
    /// --
    ///
    /// Increments the Size to account for the given padding.
    ///
    /// Arguments
    /// ---------
    /// padding : float
    ///     The other size.
    fn pad(&self, to: f32) -> PyResult<Self> {
        Ok(Self(self.0.pad(f32_nonneg(to)?)))
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str, &'static str) {
        ("width", "height")
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        let Size { width, height } = self.0;
        format_to_py!("Size({:?}, {:?})", width, height)
    }
}
