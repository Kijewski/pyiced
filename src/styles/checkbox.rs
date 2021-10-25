use iced::checkbox::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedCheckboxStyle>()?;
    Ok(())
}

/// CheckboxStyle(**kwargs)
/// --
///
/// The appearance of a checkbox.
///
/// Parameters
/// ----------
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
/// * `iced::widget::checkbox::Style <https://docs.rs/iced/0.3.0/iced/widget/checkbox/struct.Style.html>`_
#[pyclass(name = "CheckboxStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedCheckboxStyle(pub CheckboxStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct CheckboxStyle(pub Style);

#[derive(Debug, Clone, Copy)]
pub(crate) struct CheckboxStyles {
    pub active: CheckboxStyle,
    pub hovered: CheckboxStyle,
}

impl Default for CheckboxStyles {
    fn default() -> Self {
        let proto = Box::<dyn StyleSheet>::default();
        Self {
            active: CheckboxStyle(proto.active(false)),
            hovered: CheckboxStyle(proto.hovered(false)),
        }
    }
}

#[pymethods]
impl WrappedCheckboxStyle {
    #[args(kwargs = "**")]
    #[new]
    fn new(hovered: bool, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = Box::<dyn StyleSheet>::default();
        let proto = match hovered {
            false => proto.active(false),
            true => proto.hovered(false),
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

impl StyleSheet for CheckboxStyles {
    fn active(&self, _is_checked: bool) -> Style {
        self.active.0
    }

    fn hovered(&self, _is_checked: bool) -> Style {
        self.hovered.0
    }
}
