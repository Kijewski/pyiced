use iced::{Element, Rule};
use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{GCProtocol, Message, ToNative, empty_space};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_rule, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct RuleBuilder {
    pub horizontal: Option<u16>,
    pub vertical: Option<u16>,
    // style: TODO,
}

impl GCProtocol for RuleBuilder {}

#[pyfunction(name="rule")]
fn make_rule<'p>(
    horizontal: Option<u16>,
    vertical: Option<u16>,
) -> WrappedWidgetBuilder {
    RuleBuilder {
        horizontal,
        vertical,
    }.into()
}

impl ToNative for RuleBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = match self {
            &Self { horizontal: Some(spacing), vertical: None } => Rule::horizontal(spacing),
            &Self { horizontal: None, vertical: Some(spacing) } => Rule::vertical(spacing),
            _ => return empty_space(),
        };
        el.into()
    }
}
