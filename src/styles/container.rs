use iced::container::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::WrappedColor;
use crate::{extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedContainerStyle>()?;
    Ok(())
}

/// ContainerStyleSheet(proto=None, **kwargs)
/// --
///
/// The appearance of a container.
///
/// Parameters
/// ----------
/// proto : Optional[ContainerStyleSheet]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
/// text_color : Optional[Color]
///     The container's text color.
/// background : Optional[Color]
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
/// * `iced::widget::container::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/container/trait.StyleSheet.html>`_
#[pyclass(name = "ContainerStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedContainerStyle(pub ContainerStyle);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct ContainerStyle(pub Style);

#[pymethods]
impl WrappedContainerStyle {
    #[args(proto = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&Self>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = proto.map_or_else(ContainerStyle::default, |p| p.0);
        extract_multiple!(
            kwargs,
            proto,
            text_color,
            background,
            border_radius,
            border_width,
            border_color,
        )
    }
}

getters! {
    WrappedContainerStyle => |&WrappedContainerStyle(ContainerStyle(ref o))| o,
    text_color -> "Optional[Color]" Option<WrappedColor>,
    background -> "Optional[Color]" Option<WrappedColor>,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
}

impl StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        self.0
    }
}
