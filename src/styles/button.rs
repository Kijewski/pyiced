use iced::button::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::WrappedColor;
use crate::{extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedButtonStyleSheet>()?;
    Ok(())
}

/// ButtonStyleSheet(proto=None, **kwargs)
/// --
///
/// The appearance of a button.
///
/// Parameters
/// ----------
/// proto : Optional[ButtonStyleSheet]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
/// shadow_offset : Tuple[float, float]
///     The button's shadow offset.
/// background : Optional[Color]
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
/// * `iced::widget::button::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/button/trait.StyleSheet.html>`_
#[pyclass(name = "ButtonStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedButtonStyleSheet(pub ButtonStyle);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct ButtonStyle(pub Style);

#[pymethods]
impl WrappedButtonStyleSheet {
    #[args(proto = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&Self>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = proto.map_or_else(ButtonStyle::default, |p| p.0);
        extract_multiple!(
            kwargs,
            proto,
            shadow_offset,
            background,
            border_radius,
            border_width,
            border_color,
            text_color,
        )
    }
}

getters! {
    WrappedButtonStyleSheet => |&WrappedButtonStyleSheet(ButtonStyle(ref o))| o,
    shadow_offset -> "Tuple[float]" (f32, f32),
    background -> "Optional[Color]" Option<WrappedColor>,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
    text_color -> "Color" WrappedColor,
}

impl StyleSheet for ButtonStyle {
    fn active(&self) -> Style {
        self.0
    }
}
