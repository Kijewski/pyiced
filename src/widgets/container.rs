use iced::{Align, Container, Element, Length};
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WidgetBuilder;
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_container, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ContainerBuilder {
    pub content: Box<WidgetBuilder>,
    pub padding: Option<u16>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_x: Option<Align>,
    pub align_y: Option<Align>,
    // style: TODO,
}

impl GCProtocol for ContainerBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.content.traverse(visit)
    }
}

#[pyfunction(name="container")]
fn make_container(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for ContainerBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = Container::new(content);
        let el = assign!(el, self, padding, width, height, max_width, max_height, align_x, align_y);
        el.into()
    }
}
