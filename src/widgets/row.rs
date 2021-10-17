use iced::{Align, Element, Length, Row};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedAlign, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_row, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RowBuilder {
    pub children: Vec<WidgetBuilder>,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<Align>,
}

impl GCProtocol for RowBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        for child in self.children.iter() {
            child.traverse(visit)?;
        }
        Ok(())
    }
}

#[pyfunction(name = "row")]
/// row($module, /, children, *, spacing=None, padding=None, width=None, height=None, max_width=None, max_height=None, align_items=None)
/// --
///
/// A container that distributes its contents horizontally.
///
/// Parameters
/// ----------
/// children : Iterable[Optional[Element]]
///     Create the row with the given elements.
/// spacing : Optional[int]
///     Sets the horizontal spacing between elements.
/// padding : Optional[int]
///     Padding of the row.
/// width : Optional[Length]
///     Width of the row.
/// height : Optional[Length]
///     Height of the row.
/// max_width : Optional[int]
///     Maximum width of the row.
/// max_height : Optional[int]
///     Maximum height of the row.
/// align_items : Optional[Align]
///     Vertical alignment of the contents of the row.
///
/// Returns
/// -------
/// Element
///     The newly created row.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/row.png
///    :width: 688
///    :height: 405
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/row.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::row::Row <https://docs.rs/iced_native/0.4.0/iced_native/widget/row/struct.Row.html>`_
fn make_row(
    py: Python,
    children: &PyAny,
    spacing: Option<u16>,
    padding: Option<u16>,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    align_items: Option<&WrappedAlign>,
) -> PyResult<WrappedWidgetBuilder> {
    let children = children
        .iter()?
        .filter_map(|child| match child {
            Ok(child) => match child.is_none() {
                false => match child.extract() {
                    Ok(WrappedWidgetBuilder(widget)) => Some(widget),
                    Err(err) => {
                        err.print(py);
                        None
                    },
                },
                true => None,
            },
            Err(err) => {
                err.print(py);
                None
            },
        })
        .collect();
    let el = RowBuilder {
        children,
        spacing,
        padding,
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        max_width,
        max_height,
        align_items: align_items.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for RowBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let children = self
            .children
            .iter()
            .map(|child| child.to_native(py))
            .collect();
        let el = Row::with_children(children);
        let el = assign!(
            el,
            self,
            spacing,
            padding,
            width,
            height,
            max_width,
            max_height,
            align_items
        );
        el.into()
    }
}
