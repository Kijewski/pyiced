use iced::{Element, Rule};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{empty_space, GCProtocol, Message, ToNative};
use crate::styles::{RuleStyle, WrappedRuleStyleSheet};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_rule, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuleBuilder {
    pub horizontal: u16,
    pub vertical: u16,
    pub style: Option<RuleStyle>,
}

impl GCProtocol for RuleBuilder {}

#[pyfunction(name = "rule")]
/// rule($module, *, horizontal=0, vertical=0, style=None)
/// --
///
/// Display a horizontal or vertical rule for dividing content.
///
/// Parameters
/// ----------
/// horizontal : Optional[int]
///     Creates a horizontal rule for dividing content by the given vertical spacing.
/// vertical : Optional[int]
///     Creates a vertical rule for dividing content by the given horizontal spacing.
/// style : Optional[RuleStyleSheet]
///     The style of the rule.
///
/// Returns
/// -------
/// Element
///     The newly created divider.
///
/// See also
/// --------
/// `iced_native::widget::rule::Rule <https://docs.rs/iced_native/0.4.0/iced_native/widget/rule/struct.Rule.html>`_
fn make_rule(
    horizontal: Option<u16>,
    vertical: Option<u16>,
    style: Option<&WrappedRuleStyleSheet>,
) -> PyResult<WrappedWidgetBuilder> {
    let horizontal = horizontal.unwrap_or_default();
    let vertical = vertical.unwrap_or_default();
    if (horizontal != 0) != (vertical != 0) {
        return Err(PyErr::new::<PyValueError, _>(
            "You need to specify EITHER 'horizontal' OR 'vertical' with a value > 0.",
        ));
    }
    let el = RuleBuilder {
        horizontal,
        vertical,
        style: style.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for RuleBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let (el, style) = match *self {
            Self {
                horizontal: spacing,
                vertical: 0,
                style,
            } => (Rule::horizontal(spacing), style),
            Self {
                horizontal: 0,
                vertical: spacing,
                style,
            } => (Rule::vertical(spacing), style),
            _ => return empty_space(),
        };
        let el = match style {
            Some(style) => el.style(style),
            None => el,
        };
        el.into()
    }
}
