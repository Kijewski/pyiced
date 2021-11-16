use std::num::FpCategory;

use iced::widget::pane_grid::Line;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_py;
use crate::wrapped::color::ColorFormat;
use crate::wrapped::WrappedColor;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedLine>()?;
    Ok(())
}

/// Line(color, width)
/// --
///
/// A line.
///
/// It is normally used to define the highlight of something, like a split.
///
/// Parameters
/// ----------
/// color : Color
///     The color of the line.
/// width : float
///     The width of the line.
///
/// See also
/// --------
/// `iced::widget::pane_grid::Line <https://docs.rs/iced/0.3.0/iced/widget/pane_grid/struct.Line.html>`_
#[pyclass(name = "Line", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedLine(pub Line);

#[pymethods]
impl WrappedLine {
    #[new]
    fn new(color: &WrappedColor, width: f32) -> PyResult<Self> {
        let width = match width.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(PyErr::new::<PyValueError, _>("The width must be finite"));
            },
            FpCategory::Zero | FpCategory::Subnormal => 0.0f32,
            FpCategory::Normal => {
                if width < 0.0 {
                    return Err(PyErr::new::<PyValueError, _>("The width must be >= 0"));
                }
                width
            },
        };
        Ok(Self(Line {
            color: color.0,
            width,
        }))
    }

    /// The color of the line.
    ///
    /// Returns
    /// -------
    /// Color
    ///     The "color" parameter given when constructing this line.
    #[getter]
    fn color(&self) -> WrappedColor {
        WrappedColor(self.0.color)
    }

    /// The width of the line.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "width" parameter given when constructing this line.
    #[getter]
    fn width(&self) -> f32 {
        self.0.width
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str, &'static str) {
        ("color", "width")
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        let Line { ref color, width } = self.0;
        format_to_py!("Line({}, {}", ColorFormat(color), width)
    }
}
