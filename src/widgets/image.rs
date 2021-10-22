use iced::image::Handle;
use iced::{Element, Image, Length};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedImageHandle, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_image, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ImageBuilder {
    pub handle: Handle,
    pub width: Option<Length>,
    pub height: Option<Length>,
}

impl GCProtocol for ImageBuilder {}

#[pyfunction(name = "image")]
fn make_image(
    handle: &WrappedImageHandle,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    ImageBuilder {
        handle: handle.0.clone(),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
    }
    .into()
}

impl ToNative for ImageBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = Image::new(self.handle.clone());
        let el = assign!(el, self, width, height);
        el.into()
    }
}
