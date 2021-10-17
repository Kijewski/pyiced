use pyo3::types::PyList;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, ToNative};
use crate::widgets::WidgetBuilder;
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedAlign, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_row, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct RowBuilder {
    pub children: Vec<WidgetBuilder>,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<iced::Align>,
}

#[pyfunction(name="column")]
fn make_row<'p>(
    py: Python,
    children: &PyList,
    spacing: Option<u16>,
    padding: Option<u16>,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    align_items: Option<&WrappedAlign>,
) -> WrappedWidgetBuilder {
    let children = children.iter()
        .filter_map(|child| match child.extract() {
            Ok(WrappedWidgetBuilder(widget)) => Some(widget),
            Err(err) => {
                err.print(py);
                None
            }
        })
        .collect();
    RowBuilder {
        children,
        spacing,
        padding,
        width: width.map(|o| o.0.clone()),
        height: height.map(|o| o.0.clone()),
        max_width,
        max_height,
        align_items: align_items.map(|o| o.0.clone()),
    }.into()
}

impl ToNative for RowBuilder {
    fn to_native(&self, py: Python) -> iced::Element<'static, Message> {
        let children = self.children.iter().map(|child| child.to_native(py)).collect();
        let el = iced::Row::with_children(children);
        let el = assign!(el, self, spacing, padding, width, height, max_width, max_height, align_items);
        el.into()
    }
}
