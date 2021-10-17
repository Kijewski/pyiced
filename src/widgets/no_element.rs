use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, ToNative, empty_space};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_no_element, m)?)?;
    Ok(())
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NoElementBuilder;

#[pyfunction(name="no_element")]
fn make_no_element<'p>() -> WrappedWidgetBuilder {
    NoElementBuilder.into()
}

impl ToNative for NoElementBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        empty_space()
    }
}
