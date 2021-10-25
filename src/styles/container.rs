use std::sync::Arc;

use iced::container::{Style, StyleSheet};
use iced::Background;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::WrappedColor;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedContainerStyle>()?;
    Ok(())
}

/// The appearance of a container.
///
/// All parameters are named parameters and optional.
///
/// Parameters
/// ----------
/// text_color : Color
///     The container's text color.
/// background : Option[Color]
///     The container's background color.
/// border_radius : float
///     The container's border radius.
/// border_width : float
///     The container's border width.
/// border_color : Color
///     The container's border color.
///
/// See also
/// --------
/// * `iced::widget::container::Style <https://docs.rs/iced/0.3.0/iced/widget/container/struct.Style.html>`_
#[pyclass(name = "ContainerStyle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedContainerStyle(pub Arc<Style>);

#[pymethods]
impl WrappedContainerStyle {
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
                "text_color" => {
                    result.text_color = value.extract::<Option<WrappedColor>>()?.map(|c| c.0);
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

impl StyleSheet for WrappedContainerStyle {
    fn style(&self) -> Style {
        *self.0
    }
}
