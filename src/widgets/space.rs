use iced::{Element, Length, Space};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_space, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SpaceBuilder {
    pub width: Length,
    pub height: Length,
}

impl GCProtocol for SpaceBuilder {}

#[pyfunction(name = "space")]
fn make_space(
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    let width = match width {
        Some(width) => width.0,
        None => Length::Shrink,
    };
    let height = match height {
        Some(height) => height.0,
        None => Length::Shrink,
    };
    SpaceBuilder { width, height }.into()
}

impl ToNative for SpaceBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = Space::new(self.width, self.height);
        el.into()
    }
}
