#![allow(clippy::needless_option_as_deref)]

use iced::radio::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::wrapped::WrappedColor;
use crate::{dyn_style_proto, extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedRadioStyle>()?;
    m.add_class::<WrappedRadioStyleSheet>()?;
    Ok(())
}

/// RadioStyle(proto=None, **kwargs)
/// --
///
/// The appearance of a radio button for some state.
///
/// Parameters
/// ----------
/// proto : Optional[Union[RadioStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active" and "hovered",
///     same as the argument for :class:`~pyiced.RadioStyleSheet`.
///
///     None is the same as "active".
/// background : Color
///     The radio's background color.
/// dot_color : Color
///     The color of the dot.
/// border_width : float
///     The radio's border width.
/// border_color : Color
///     The radio's border color.
///
/// See also
/// --------
/// `iced::widget::radio::Style <https://docs.rs/iced/0.3.0/iced/widget/radio/struct.Style.html>`_
#[pyclass(name = "RadioStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedRadioStyle(pub RadioStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct RadioStyle(pub Style);

#[pymethods]
impl WrappedRadioStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = dyn_style_proto!(proto, active, hovered);
        extract_multiple!(
            kwargs,
            RadioStyle(proto),
            background,
            dot_color,
            border_width,
            border_color
        )
    }
}

getters! {
    WrappedRadioStyle => |&WrappedRadioStyle(RadioStyle(ref o))| o,
    background -> "Color" WrappedColor,
    dot_color -> "Color" WrappedColor,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
}

/// RadioStyleSheet(active, hovered=None)
/// --
///
/// The appearance of a radio.
///
/// Parameters
/// ----------
/// active : RadioStyle
///     Normal style of the radio.
/// hovered : Optional[RadioStyle]
///     Style of the radio when the cursor is hovering over it. Defaults to "active".
///
/// See also
/// --------
/// `iced::widget::radio::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/radio/trait.StyleSheet.html>`_
#[pyclass(name = "RadioStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedRadioStyleSheet(pub RadioStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct RadioStyleSheet {
    active: Style,
    hovered: Style,
}

getters! {
    WrappedRadioStyleSheet => |&WrappedRadioStyleSheet(ref o)| o,
    active -> "Color" WrappedRadioStyle,
    hovered -> "Color" WrappedRadioStyle,
}

#[pymethods]
impl WrappedRadioStyleSheet {
    #[new]
    fn new(active: &WrappedRadioStyle, hovered: Option<&WrappedRadioStyle>) -> Self {
        let active = active.0.0;
        let hovered = hovered.map_or(active, |s| s.0.0);
        Self(RadioStyleSheet { active, hovered })
    }
}

impl StyleSheet for RadioStyleSheet {
    fn active(&self) -> Style {
        self.active
    }

    fn hovered(&self) -> Style {
        self.hovered
    }
}
