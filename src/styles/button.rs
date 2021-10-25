use std::sync::Arc;

use iced::button::{Style, StyleSheet};
use iced::{Background, Vector};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::WrappedColor;

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
#[derive(Debug, Clone)]
pub(crate) struct WrappedButtonStyle(pub Arc<Style>);

#[pymethods]
impl WrappedButtonStyle {
    #[args(kwargs = "**")]
    #[new]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut result = Style::default();
        let kwargs = match kwargs {
            Some(kwargs) => kwargs,
            None => return Ok(Self(Arc::new(result))),
        };

        for (key, value) in kwargs.iter() {
            let key = key.str()?;
            match key.to_str()? {
                "shadow_offset" => {
                    let (x, y) = value.extract::<(f32, f32)>()?;
                    if !x.is_finite() || !y.is_finite() {
                        return Err(PyErr::new::<PyValueError, _>(
                            "shadow_offset x and y must be finite",
                        ));
                    }
                    result.shadow_offset = Vector { x, y };
                },
                "background" => {
                    result.background = value
                        .extract::<Option<WrappedColor>>()?
                        .map(|c| Background::Color(c.0));
                },
                "border_radius" => {
                    let value = value.extract::<f32>()?;
                    if !value.is_finite() {
                        return Err(PyErr::new::<PyValueError, _>(
                            "border_radius must be finite",
                        ));
                    }
                    result.border_radius = value;
                },
                "border_width" => {
                    let value = value.extract::<f32>()?;
                    if !value.is_finite() {
                        return Err(PyErr::new::<PyValueError, _>("border_width must be finite"));
                    }
                    result.border_width = value;
                },
                "border_color" => {
                    result.border_color = value.extract::<WrappedColor>()?.0;
                },
                "text_color" => {
                    result.text_color = value.extract::<WrappedColor>()?.0;
                },
                key => {
                    return Err(PyErr::new::<PyValueError, _>(format!(
                        "Unknown keyword argument: {:#?}",
                        key,
                    )));
                },
            }
        }

        Ok(Self(Arc::new(result)))
    }
}

impl StyleSheet for WrappedButtonStyle {
    fn active(&self) -> Style {
        *self.0
    }
}
