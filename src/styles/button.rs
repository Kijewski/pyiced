use std::sync::Arc;

use iced::button::{Style, StyleSheet};
use iced::{Background, Vector};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::wrapped::WrappedColor;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedButtonStyle>()?;
    Ok(())
}

/// The appearance of a button.
/// 
/// Parameters
/// ----------
/// shadow_offset : Optional[Tuple[float, float]]
///     The button's shadow offset.
/// background : Option[Color]
///     The button's background color.
/// border_radius : Optional[float]
///     The button's border radius.
/// border_width : Optional[float]
///     The button's border width. 
/// border_color : Optional[Color]
///     The button's border color.
/// text_color : Optional[Color]
///     The button's text color.
///
/// See also
/// --------
/// * `iced::widget::button::Style <https://docs.rs/iced/0.3.0/iced/widget/button/struct.Style.html>`_
#[pyclass(name = "ButtonStyle", module = "pyiced")]
#[derive(Default, Debug, Clone)]
pub(crate) struct WrappedButtonStyle(pub Arc<Style>);

#[pymethods]
impl WrappedButtonStyle {
    #[new]
    fn new(
        shadow_offset: Option<(f32, f32)>,
        background: Option<&WrappedColor>,
        border_radius: Option<f32>,
        border_width: Option<f32>,
        border_color: Option<&WrappedColor>,
        text_color: Option<&WrappedColor>,
    ) -> PyResult<Self> {
        let mut result = Style::default();

        if let Some((x, y)) = shadow_offset {
            if !x.is_finite() || !y.is_finite() {
                return Err(PyErr::new::<PyValueError, _>("shadow_offset x and y must be finite"));
            }
            result.shadow_offset = Vector { x, y };
        }

        if let Some(value) = background {
            result.background = Some(Background::Color(value.0));
        }

        if let Some(value) = border_radius {
            if !value.is_finite() {
                return Err(PyErr::new::<PyValueError, _>("border_radius must be finite"));
            }
            result.border_radius = value;
        }

        if let Some(value) = border_width {
            if !value.is_finite() {
                return Err(PyErr::new::<PyValueError, _>("border_width must be finite"));
            }
            result.border_width = value;
        }

        if let Some(value) = border_color {
            result.border_color = value.0;
        }

        if let Some(value) = text_color {
            result.text_color = value.0;
        }

        Ok(Self(Arc::new(result)))
    }
}

impl StyleSheet for WrappedButtonStyle {
    fn active(&self) -> Style {
        *self.0
    }
}
