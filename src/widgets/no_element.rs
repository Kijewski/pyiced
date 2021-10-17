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
/// no_element($module, /, content, *, padding=None, width=None, height=None, max_width=None, max_height=None, align_x=None, align_y=None)
/// --
///
/// A :func:`~pyiced.space` with minimum width and height.
///
/// You should never actually need to use this function is code.
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
