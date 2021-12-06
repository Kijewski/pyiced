use iced::button::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::WrappedColor;
use crate::{extract_multiple, getters, partially_defaulted_stylesheet};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedButtonStyle>()?;
    m.add_class::<WrappedButtonStyleSheet>()?;
    Ok(())
}

/// ButtonStyle(proto=None, **kwargs)
/// --
///
/// The appearance of a :func:`~pyiced.button()` for a given state.
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
/// `iced::widget::button::Style <https://docs.rs/iced/0.3.0/iced/widget/button/struct.Style.html>`_
#[pyclass(name = "ButtonStyle", module = "pyiced")]
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedButtonStyle(pub ButtonStyle);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct ButtonStyle(pub Style);

#[pymethods]
impl WrappedButtonStyle {
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
    WrappedButtonStyle => |&WrappedButtonStyle(ButtonStyle(ref o))| o,
    shadow_offset -> "Tuple[float]" (f32, f32),
    background -> "Optional[Color]" Option<WrappedColor>,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
    text_color -> "Color" WrappedColor,
}

/// ButtonStyleSheet(active, hovered=None, pressed=None, disabled=None)
/// --
///
/// The appearance of a :func:`~pyiced.button()`.
///
/// Parameters
/// ----------
/// active : ButtonStyle
///     Normal style of the button.
/// hovered : Optional[ButtonStyle]
///     Style of the button when the cursor is hovering over it. Defaults to a style derived from "active".
/// pressed : Optional[ButtonStyle]
///     Style of the button while it's pressed down. Defaults to a style derived from "active".
/// disabled : Optional[ButtonStyle]
///     Style of the button when no "on_press" argument was given. Defaults to a style derived from "active".
///
/// See also
/// --------
/// `iced::widget::button::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/button/trait.StyleSheet.html>`_
#[pyclass(name = "ButtonStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedButtonStyleSheet(pub ButtonStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ButtonStyleSheet {
    active: Style,
    hovered: Style,
    pressed: Style,
    disabled: Style,
}

getters! {
    WrappedButtonStyleSheet => |&WrappedButtonStyleSheet(ref o)| o,
    active -> "ButtonStyle" WrappedButtonStyle,
    hovered -> "ButtonStyle" WrappedButtonStyle,
    pressed -> "ButtonStyle" WrappedButtonStyle,
    disabled -> "ButtonStyle" WrappedButtonStyle,
}

#[pymethods]
impl WrappedButtonStyleSheet {
    #[new]
    fn new(
        active: &WrappedButtonStyle,
        hovered: Option<&WrappedButtonStyle>,
        pressed: Option<&WrappedButtonStyle>,
        disabled: Option<&WrappedButtonStyle>,
    ) -> Self {
        let active = active.0.0;
        partially_defaulted_stylesheet!(Style, StyleSheet, active => hovered);
        partially_defaulted_stylesheet!(Style, StyleSheet, active => pressed);
        partially_defaulted_stylesheet!(Style, StyleSheet, active => disabled);
        Self(ButtonStyleSheet {
            active,
            hovered,
            pressed,
            disabled,
        })
    }
}

impl StyleSheet for ButtonStyleSheet {
    fn active(&self) -> Style {
        self.active
    }

    fn hovered(&self) -> Style {
        self.hovered
    }

    fn pressed(&self) -> Style {
        self.pressed
    }

    fn disabled(&self) -> Style {
        self.disabled
    }
}
