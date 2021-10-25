use iced::container::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedContainerStyle>()?;
    Ok(())
}

/// ContainerStyle(**kwargs)
/// --
///
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
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedContainerStyle(pub ContainerStyle);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct ContainerStyle(pub Style);

#[pymethods]
impl WrappedContainerStyle {
    #[args(kwargs = "**")]
    #[new]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        extract_multiple!(
            kwargs,
            ContainerStyle::default(),
            text_color,
            background,
            border_radius,
            border_width,
            border_color,
        )
    }
}

impl StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        self.0
    }
}
