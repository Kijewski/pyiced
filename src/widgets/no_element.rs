use iced::Element;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{GCProtocol, Message, ToNative, empty_space};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_no_element, m)?)?;
    Ok(())
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NoElementBuilder;

impl GCProtocol for NoElementBuilder {}

#[pyfunction(name="no_element")]
fn make_no_element<'p>() -> WrappedWidgetBuilder {
    NoElementBuilder.into()
}

impl ToNative for NoElementBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        empty_space()
    }
}
