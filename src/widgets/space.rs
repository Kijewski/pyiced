use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_space, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SpaceBuilder {
    pub width: iced::Length,
    pub height: iced::Length,
}

#[pyfunction(name="space")]
fn make_space<'p>(
    width: &WrappedLength,
    height: &WrappedLength,
) -> WrappedWidgetBuilder {
    SpaceBuilder {
        width: width.0,
        height: height.0,
    }.into()
}

impl ToNative for SpaceBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Space::new(self.width, self.height);
        el.into()
    }
}
