#![allow(clippy::needless_option_as_deref)]

use iced::checkbox::{Style, StyleSheet};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::common::debug_err;
use crate::wrapped::WrappedColor;
use crate::{extract_multiple, format_to_string_ignore, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedCheckboxStyle>()?;
    m.add_class::<WrappedCheckboxStyleSheet>()?;
    Ok(())
}

/// CheckboxStyle(proto=None, **kwargs)
/// --
///
/// The appearance of a checkbox for some state.
///
/// Parameters
/// ----------
/// proto : Optional[Union[CheckboxStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active", "hovered", "active_checked" and "hovered_checked",
///     same as the argument for :class:`pyiced.~CheckboxStyleSheet`.
///
///     None is the same as "active".
/// background : Color
///     The checkbox' background color.
/// checkmark_color : Color
///     The color of the checkbox.
/// border_radius : float
///     The checkbox' border radius.
/// border_width : float
///     The checkbox' border width.
/// border_color : Color
///     The checkbox' border color.
///
/// See also
/// --------
/// `iced::widget::checkbox::Style <https://docs.rs/iced/0.3.0/iced/widget/checkbox/struct.Style.html>`_
#[pyclass(name = "CheckboxStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedCheckboxStyle(pub CheckboxStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct CheckboxStyle(pub Style);

#[pymethods]
impl WrappedCheckboxStyle {
    #[args(proto = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = match proto {
            Some(proto) => match proto.extract() {
                Ok(Self(proto)) => proto.0,
                Err(_) => match proto.downcast::<PyString>() {
                    Ok(s) => match s.to_str()? {
                        "active" => Box::<dyn StyleSheet>::default().active(false),
                        "hovered" => Box::<dyn StyleSheet>::default().hovered(false),
                        "active_checked" => Box::<dyn StyleSheet>::default().active(true),
                        "hovered_checked" => Box::<dyn StyleSheet>::default().hovered(true),
                        s => {
                            return Err(PyErr::new::<PyValueError, _>(format_to_string_ignore!(
                                "Unknown proto value: {:#}",
                                s
                            )));
                        },
                    },
                    Err(err) => {
                        return Err(debug_err::<PyTypeError, _>(err));
                    },
                },
            },
            None => Box::<dyn StyleSheet>::default().active(false),
        };

        extract_multiple!(
            kwargs,
            CheckboxStyle(proto),
            background,
            checkmark_color,
            border_radius,
            border_width,
            border_color,
        )
    }
}

getters! {
    WrappedCheckboxStyle => |&WrappedCheckboxStyle(CheckboxStyle(ref o))| o,
    background -> "Color" WrappedColor,
    checkmark_color -> "Color" WrappedColor,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
}

/// CheckboxStyleSheet(active, hoverered=None, active_checked=None, hovered_checked=None)
/// --
///
/// The appearance of a checkbox.
///
/// Parameters
/// ----------
/// active : CheckboxStyle
///     Normal style of this checkbox.
/// hovered : Optional[CheckboxStyle]
///     Style when hovering over the checkbox. Defaults to the same style as "active".
/// active_checked : Optional[CheckboxStyle]
///     Style of this checkbox when the checkbox is checked. Defaults to the same style as "active".
/// hovered_checked : Optional[CheckboxStyle]
///     Style when hovering over the checked checkbox.
///     If None or absent, it defaults to the first argument with an explicit value in
///     "hovered", "active_checked" or "active".
///
/// See also
/// --------
/// `iced::widget::checkbox::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/checkbox/trait.StyleSheet.html>`_
#[pyclass(name = "CheckboxStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedCheckboxStyleSheet(pub CheckboxStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct CheckboxStyleSheet {
    pub active: Style,
    pub hovered: Style,
    pub active_checked: Style,
    pub hovered_checked: Style,
}

getters! {
    WrappedCheckboxStyleSheet => |&WrappedCheckboxStyleSheet(ref o)| o,
    active -> "CheckboxStyle" WrappedCheckboxStyle,
    hovered -> "CheckboxStyle" WrappedCheckboxStyle,
    active_checked -> "CheckboxStyle" WrappedCheckboxStyle,
    hovered_checked -> "CheckboxStyle" WrappedCheckboxStyle,
}

#[pymethods]
impl WrappedCheckboxStyleSheet {
    #[new]
    fn new(
        active: &WrappedCheckboxStyle,
        hovered: Option<&WrappedCheckboxStyle>,
        active_checked: Option<&WrappedCheckboxStyle>,
        hovered_checked: Option<&WrappedCheckboxStyle>,
    ) -> Self {
        let active = active.0.0;
        let hovered_checked = [hovered_checked, hovered, active_checked]
            .iter()
            .find_map(|o| o.map(|o| o.0.0))
            .unwrap_or(active);
        let hovered = hovered.map_or(active, |s| s.0.0);
        let active_checked = active_checked.map_or(active, |s| s.0.0);
        Self(CheckboxStyleSheet {
            active,
            hovered,
            active_checked,
            hovered_checked,
        })
    }
}

impl StyleSheet for CheckboxStyleSheet {
    fn active(&self, is_checked: bool) -> Style {
        match is_checked {
            true => self.active_checked,
            false => self.active,
        }
    }

    fn hovered(&self, is_checked: bool) -> Style {
        match is_checked {
            true => self.hovered_checked,
            false => self.hovered,
        }
    }
}
