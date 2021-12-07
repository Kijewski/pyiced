use iced::Element;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{empty_space, GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_no_element, m)?)?;
    Ok(())
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NoElementBuilder;

impl GCProtocol for NoElementBuilder {}

#[pyfunction(name = "no_element")]
/// no_element()
/// --
///
/// A :func:`~pyiced.space()` with minimum width and height.
///
/// Returns
/// -------
/// Element
///     The newly created empty space.
fn make_no_element() -> WrappedWidgetBuilder {
    NoElementBuilder.into()
}

impl ToNative for NoElementBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        empty_space()
    }
}
