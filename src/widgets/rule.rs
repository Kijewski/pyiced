use iced::{Element, Rule};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{empty_space, GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_rule, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuleBuilder {
    pub horizontal: Option<u16>,
    pub vertical: Option<u16>,
    // style: TODO,
}

impl GCProtocol for RuleBuilder {}

#[pyfunction(name = "rule")]
/// rule($module, *, horizontal=0, vertical=0)
/// --
///
/// Display a horizontal or vertical rule for dividing content.
///
/// Parameters
/// ----------
/// horizontal : int
///     Creates a horizontal rule for dividing content by the given vertical spacing.
/// vertical : int
///     Creates a vertical rule for dividing content by the given horizontal spacing.
///
/// Returns
/// -------
/// Element
///     The newly created divider.
///
/// See also
/// --------
/// `iced_native::widget::rule::Rule <https://docs.rs/iced_native/0.4.0/iced_native/widget/rule/struct.Rule.html>`_
fn make_rule(horizontal: Option<u16>, vertical: Option<u16>) -> PyResult<WrappedWidgetBuilder> {
    let horizontal = horizontal.and_then(|v| match v {
        0 => None,
        v => Some(v),
    });
    let vertical = vertical.and_then(|v| match v {
        0 => None,
        v => Some(v),
    });
    if horizontal.is_some() != vertical.is_some() {
        return Err(PyErr::new::<PyValueError, _>(
            "You need to specify EITHER 'horizontal' OR 'vertical' with a value > 0.",
        ));
    }
    let el = RuleBuilder {
        horizontal,
        vertical,
    };
    Ok(el.into())
}

impl ToNative for RuleBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = match *self {
            Self {
                horizontal: Some(spacing),
                vertical: None,
            } => Rule::horizontal(spacing),
            Self {
                horizontal: None,
                vertical: Some(spacing),
            } => Rule::vertical(spacing),
            _ => return empty_space(),
        };
        el.into()
    }
}
