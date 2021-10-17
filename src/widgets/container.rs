use iced::{Align, Container, Element, Length};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::styles::{ContainerStyle, WrappedContainerStyle};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedAlign, WrappedLength};

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
    pub style: Option<ContainerStyle>,
}

impl GCProtocol for ContainerBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.content.traverse(visit)
    }
}

#[pyfunction(name = "container")]
/// container($module, /, content, *, padding=None, width=None, height=None, max_width=None, max_height=None, align_x=None, align_y=None, style=None)
/// --
///
/// Make a .
///
/// Parameters
/// ----------
///
/// Returns
/// -------
/// Element
///     The newly created .
///
/// See also
/// --------
/// `iced_native::widget::container::Container <https://docs.rs/iced_native/0.4.0/iced_native/widget/container/struct.Container.html>`_
fn make_container(
    content: &WrappedWidgetBuilder,
    padding: Option<u16>,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    align_x: Option<&WrappedAlign>,
    align_y: Option<&WrappedAlign>,
    style: Option<&WrappedContainerStyle>,
) -> WrappedWidgetBuilder {
    let el = ContainerBuilder {
        content: Box::new(content.0.clone()),
        padding,
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        max_width,
        max_height,
        align_x: align_x.map(|o| o.0),
        align_y: align_y.map(|o| o.0),
        style: style.map(|o| o.0),
    };
    el.into()
}

impl ToNative for ContainerBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = Container::new(content);
        let el = assign!(
            el, self, padding, width, height, max_width, max_height, align_x, align_y, style,
        );
        el.into()
    }
}
