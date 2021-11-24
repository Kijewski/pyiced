use iced::rule::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::{WrappedColor, WrappedFillMode};
use crate::{extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedRuleStyleSheet>()?;
    Ok(())
}

/// RuleStyleSheet(proto=None, **kwargs)
/// --
///
/// The appearance of a rule.
///
/// Parameters
/// ----------
/// proto : Optional[RuleStyleSheet]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
/// color : Color
///     The color of the rule.
/// width : int
///     The width (thickness) of the rule line.
/// radius : float
///     The radius of the line corners.
/// fill_mode : FillMode
///     The fill mode of the rule.
///
/// See also
/// --------
/// * `iced::widget::rule::Style <https://docs.rs/iced/0.3.0/iced/widget/rule/struct.Style.html>`_
/// * `iced::widget::rule::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/rule/trait.StyleSheet.html>`_
#[pyclass(name = "RuleStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedRuleStyleSheet(pub RuleStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct RuleStyle(pub Style);

#[pymethods]
impl WrappedRuleStyleSheet {
    #[args(proto = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&Self>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = RuleStyle(match proto {
            Some(proto) => proto.0.0,
            None => Box::<dyn StyleSheet>::default().style(),
        });
        extract_multiple!(kwargs, proto, color, width, radius, fill_mode)
    }
}

getters! {
    WrappedRuleStyleSheet => |&WrappedRuleStyleSheet(RuleStyle(ref o))| o,
    color -> "Color" WrappedColor,
    width -> "Color" u16,
    radius -> "float" f32,
    fill_mode -> "FillMode" WrappedFillMode,
}

impl StyleSheet for RuleStyle {
    fn style(&self) -> Style {
        self.0
    }
}
