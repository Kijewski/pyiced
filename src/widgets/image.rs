use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedLength, WrappedImageHandle};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_image, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ImageBuilder {
    pub handle: iced::image::Handle,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
}

#[pyfunction(name="image")]
fn make_image<'p>(
    handle: WrappedImageHandle,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    ImageBuilder {
        handle: handle.0.clone(),
        width: width.map(|o| o.0.clone()),
        height: height.map(|o| o.0.clone()),
    }.into()
}

impl ToNative for ImageBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Image::new(self.handle.clone());
        let el = assign!(el, self, width, height);
        el.into()
    }
}
