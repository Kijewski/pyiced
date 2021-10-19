use iced::svg::Handle;
use iced::{Element, Length, Svg};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{empty_space, GCProtocol, Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedLength, WrappedSvgHandle};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_svg, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SvgBuilder {
    pub handle: NonOptional<Handle>,
    pub width: Option<Length>,
    pub height: Option<Length>,
}

impl GCProtocol for SvgBuilder {}

#[pyfunction(name = "svg")]
fn make_svg(
    handle: &WrappedSvgHandle,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    SvgBuilder {
        handle: Some(handle.0.clone()),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
    }
    .into()
}

impl ToNative for SvgBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let handle = match &self.handle {
            Some(handle) => handle.clone(),
            None => return empty_space(),
        };
        let el = Svg::new(handle);
        let el = assign!(el, self, width, height);
        el.into()
    }
}
