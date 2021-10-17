use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, ToNative, empty_space};
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

#[pyfunction(name="rule")]
fn make_rule<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for RuleBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let el = match self {
            &Self { horizontal: Some(spacing), vertical: None } => iced::Rule::horizontal(spacing),
            &Self { horizontal: None, vertical: Some(spacing) } => iced::Rule::vertical(spacing),
            _ => return empty_space(),
        };
        el.into()
    }
}
