use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, ToNative};
use crate::widgets::WidgetBuilder;
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_container, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ContainerBuilder {
    pub content: Box<WidgetBuilder>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_x: Option<iced::Align>,
    pub align_y: Option<iced::Align>,
    // style: TODO,
}

#[pyfunction(name="container")]
fn make_container<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for ContainerBuilder {
    fn to_native(&self, py: Python) -> iced::Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = iced::Container::new(content);
        let el = assign!(el, self, padding, width, height, max_width, max_height, align_x, align_y);
        el.into()
    }
}
