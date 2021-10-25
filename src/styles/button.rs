use iced::button::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedButtonStyle>()?;
    Ok(())
}

/// The appearance of a button.
///
/// All parameters are named parameters and optional.
///
/// Parameters
/// ----------
/// shadow_offset : Tuple[float, float]
///     The button's shadow offset.
/// background : Option[Color]
///     The button's background color.
/// border_radius : float
///     The button's border radius.
/// border_width : float
///     The button's border width.
/// border_color : Color
///     The button's border color.
/// text_color : Color
///     The button's text color.
///
/// See also
/// --------
/// * `iced::widget::button::Style <https://docs.rs/iced/0.3.0/iced/widget/button/struct.Style.html>`_
#[pyclass(name = "ButtonStyle", module = "pyiced")]
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedButtonStyle(pub ButtonStyle);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct ButtonStyle(pub Style);

#[pymethods]
impl WrappedButtonStyle {
    #[args(kwargs = "**")]
    #[new]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        extract_multiple!(
            kwargs,
            ButtonStyle::default(),
            shadow_offset,
            background,
            border_radius,
            border_width,
            border_color,
            text_color,
        )
    }
}

impl StyleSheet for ButtonStyle {
    fn active(&self) -> Style {
        self.0
    }
}
