use iced::{Align, Column, Element, Length};
use pyo3::types::PyList;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedLength, WrappedAlign};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_column, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ColumnBuilder {
    pub children: Vec<WidgetBuilder>,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<Align>,
}

impl GCProtocol for ColumnBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        for child in self.children.iter() {
            child.traverse(visit)?;
        }
        Ok(())
    }
}

#[pyfunction(name="column")]
fn make_column(
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
    ColumnBuilder {
        children,
        spacing,
        padding,
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        max_width,
        max_height,
        align_items: align_items.map(|o| o.0),
    }.into()
}

impl ToNative for ColumnBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let children = self.children.iter().map(|child| child.to_native(py)).collect();
        let el = Column::with_children(children);
        let el = assign!(el, self, spacing, padding, width, height, max_width, max_height, align_items);
        el.into()
    }
}
