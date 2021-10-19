use iced::{Element, Length, Space};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{empty_space, GCProtocol, Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_space, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SpaceBuilder {
    pub width: NonOptional<Length>,
    pub height: NonOptional<Length>,
}

impl GCProtocol for SpaceBuilder {}

#[pyfunction(name = "space")]
fn make_space(width: &WrappedLength, height: &WrappedLength) -> WrappedWidgetBuilder {
    SpaceBuilder {
        width: Some(width.0),
        height: Some(height.0),
    }
    .into()
}

impl ToNative for SpaceBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let (width, height) = match (self.width, self.height) {
            (Some(width), Some(height)) => (width, height),
            _ => return empty_space(),
        };
        let el = Space::new(width, height);
        el.into()
    }
}
