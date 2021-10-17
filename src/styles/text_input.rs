#![allow(clippy::needless_option_as_deref)]

use iced::text_input::{Style, StyleSheet};
use iced::Color;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::wrapped::WrappedColor;
use crate::{dyn_style_proto, extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTextInputStyle>()?;
    m.add_class::<WrappedTextInputStyleSheet>()?;
    Ok(())
}

/// TextInputStyle(proto=None, **kwargs)
/// --
///
/// The appearance of a :func:`~pyiced.text_input()` for some state.
///
/// Parameters
/// ----------
/// proto : Optional[Union[TextInputStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active", "focused" and "hovered",
///     same as the argument for :class:`~pyiced.TextInputStyleSheet`.
///
///     None is the same as "active".
/// background : Color
///     The text_input's background color.
/// border_radius : float
///     The text_input's border radius.
/// border_width : float
///     The text_input's border width.
/// border_color : Color
///     The text_input's border color.
///
/// See also
/// --------
/// `iced::widget::text_input::Style <https://docs.rs/iced/0.3.0/iced/widget/text_input/struct.Style.html>`_
#[pyclass(name = "TextInputStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedTextInputStyle(pub TextInputStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct TextInputStyle(pub Style);

#[pymethods]
impl WrappedTextInputStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = dyn_style_proto!(proto, active, focused, hovered);
        extract_multiple!(
            kwargs,
            TextInputStyle(proto),
            background,
            border_radius,
            border_width,
            border_color,
        )
    }
}

getters! {
    WrappedTextInputStyle => |&WrappedTextInputStyle(TextInputStyle(ref o))| o,
    background -> "Color" WrappedColor,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
}

/// TextInputStyleSheet(active, focused=None, hovered=None, placeholder_color=None, value_color=None, selection_color=None)
/// --
///
/// The appearance of a :func:`~pyiced.text_input()`.
///
/// Parameters
/// ----------
/// active : TextInputStyle
///     Normal style of the text_input.
/// focused : Optional[TextInputStyle]
///     Style of the text_input when the cursor is hovering over it. Defaults to "active".
/// hovered : Optional[TextInputStyle]
///     Style of the text_input is being dragged. Defaults to "focused".
/// placeholder_color : Optional[Color]
///     Text color of the placeholder text.
/// value_color : Optional[Color]
///     Color of the text.
/// selection_color : Optional[Color]
///     Color of the selection.
///
/// See also
/// --------
/// `iced::widget::text_input::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/text_input/trait.StyleSheet.html>`_
#[pyclass(name = "TextInputStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedTextInputStyleSheet(pub TextInputStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct TextInputStyleSheet {
    active: Style,
    focused: Style,
    hovered: Style,
    placeholder_color: Color,
    value_color: Color,
    selection_color: Color,
}

getters! {
    WrappedTextInputStyleSheet => |&WrappedTextInputStyleSheet(ref o)| o,
    active -> "TextInputStyle" WrappedTextInputStyle,
    focused -> "TextInputStyle" WrappedTextInputStyle,
    hovered -> "TextInputStyle" WrappedTextInputStyle,
    placeholder_color -> "Color" WrappedColor,
    value_color -> "Color" WrappedColor,
    selection_color -> "Color" WrappedColor,
}

#[pymethods]
impl WrappedTextInputStyleSheet {
    #[new]
    fn new(
        active: &WrappedTextInputStyle,
        focused: Option<&WrappedTextInputStyle>,
        hovered: Option<&WrappedTextInputStyle>,
        placeholder_color: Option<&WrappedColor>,
        value_color: Option<&WrappedColor>,
        selection_color: Option<&WrappedColor>,
    ) -> Self {
        let active = active.0.0;
        let focused = focused.map_or(active, |s| s.0.0);
        let hovered = hovered.map_or(focused, |s| s.0.0);
        let colors = match (placeholder_color, value_color, selection_color) {
            (Some(a), Some(b), Some(c)) => (a.0, b.0, c.0),
            (a, b, c) => {
                let dflt = Box::<dyn StyleSheet>::default();
                let a = a.map_or_else(|| dflt.placeholder_color(), |s| s.0);
                let b = b.map_or_else(|| dflt.value_color(), |s| s.0);
                let c = c.map_or_else(|| dflt.selection_color(), |s| s.0);
                (a, b, c)
            },
        };
        let (placeholder_color, value_color, selection_color) = colors;
        Self(TextInputStyleSheet {
            active,
            focused,
            hovered,
            placeholder_color,
            value_color,
            selection_color,
        })
    }
}

impl StyleSheet for TextInputStyleSheet {
    fn active(&self) -> Style {
        self.active
    }

    fn focused(&self) -> Style {
        self.focused
    }

    fn hovered(&self) -> Style {
        self.hovered
    }

    fn placeholder_color(&self) -> Color {
        self.placeholder_color
    }

    fn value_color(&self) -> Color {
        self.value_color
    }

    fn selection_color(&self) -> Color {
        self.selection_color
    }
}
