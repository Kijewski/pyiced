use iced::{Element, Font, Tooltip};
use iced::tooltip::Position;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedFont, WrappedTooltipPosition};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_tooltip, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct TooltipBuilder {
    pub content: Box<WidgetBuilder>,
    pub tooltip: String,
    pub position: Position,
    pub size: Option<u16>,
    pub font: Option<Font>,
    pub gap: Option<u16>,
    pub padding: Option<u16>,
    // style: TODO,
}

impl GCProtocol for TooltipBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.content.traverse(visit)
    }

    fn clear(&mut self) {
        *self.content = WidgetBuilder::NoElement(Default::default());
    }
}

#[pyfunction(name="tooltip")]
fn make_tooltip<'p>(
    content: &WrappedWidgetBuilder,
    tooltip: String,
    position: &WrappedTooltipPosition,
    size: Option<u16>,
    font: Option<&WrappedFont>,
    gap: Option<u16>,
    padding: Option<u16>,
) -> WrappedWidgetBuilder {
    TooltipBuilder {
        content: Box::new(content.0.clone()),
        tooltip,
        position: position.0.clone(),
        size,
        font: font.map(|o| o.0.clone()),
        gap,
        padding,
    }.into()
}

impl ToNative for TooltipBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = Tooltip::new(content, &self.tooltip, self.position);
        let el = assign!(el, self, size, font, gap, padding);
        el.into()
    }
}
