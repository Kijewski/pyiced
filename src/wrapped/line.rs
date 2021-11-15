use iced::widget::pane_grid::Line;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_py;
use crate::wrapped::WrappedColor;
use crate::wrapped::color::ColorFormat;

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
        if !width.is_finite() || width < 0.0 {
            return Err(PyErr::new::<PyValueError, _>(
                "The width must be finite and >= 0",
            ));
        }
        Ok(Self(Line {
            color: color.0,
            width,
        }))
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        let Line { ref color, width } = self.0;
        format_to_py!("Line({}, {}", ColorFormat(color), width)
    }
}
