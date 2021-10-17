use iced::tooltip::Position;
use iced::{Element, Font, Tooltip};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

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
}

#[pyfunction(name = "tooltip")]
/// tooltip($module, /, content, tooltip, position, *, font, gap, padding)
/// --
///
/// Make a tooltip.
///
/// Parameters
/// ----------
/// content : Element
///     Contained element that has a tooltip.
/// tooltip : str
///     Tooltip text to display.
/// position : TooltipPosition
///     The position of the tooltip.
/// size : Optional[int]
///     The size of the text of the tooltip.
/// font : Optional[Font]
///     The font of the tooltip.
/// gap : Optional[int]
///     The gap between the content and its tooltip.
/// padding : Optional[int]
///
/// Returns
/// -------
/// Element
///     The newly created tooltip.
///
/// See also
/// --------
/// `iced_native::widget::tooltip::Tooltip <https://docs.rs/iced_native/0.4.0/iced_native/widget/tooltip/struct.Tooltip.html>`_
fn make_tooltip(
    content: &WrappedWidgetBuilder,
    tooltip: String,
    position: &WrappedTooltipPosition,
    size: Option<u16>,
    font: Option<&WrappedFont>,
    gap: Option<u16>,
    padding: Option<u16>,
) -> WrappedWidgetBuilder {
    let el = TooltipBuilder {
        content: Box::new(content.0.clone()),
        tooltip,
        position: position.0,
        size,
        font: font.map(|o| o.0),
        gap,
        padding,
    };
    el.into()
}

impl ToNative for TooltipBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = Tooltip::new(content, &self.tooltip, self.position);
        let el = assign!(el, self, size, font, gap, padding);
        el.into()
    }
}
