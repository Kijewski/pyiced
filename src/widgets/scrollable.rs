use iced::{Align, Element, Length, Scrollable};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, NonOptional, ToNative};
use crate::states::{scrollable_with_state, ScrollableState, WrappedScrollableState};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedAlign, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_scrollbar, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ScrollableBuilder {
    pub state: NonOptional<ScrollableState>,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<Align>,
    pub scrollbar_width: Option<u16>,
    pub scrollbar_margin: Option<u16>,
    pub scroller_width: Option<u16>,
    // style: TODO,
}

impl GCProtocol for ScrollableBuilder {}

#[pyfunction(name = "scrollbar")]
fn make_scrollbar(
    state: &WrappedScrollableState,
    spacing: Option<u16>,
    padding: Option<u16>,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    align_items: Option<&WrappedAlign>,
    scrollbar_width: Option<u16>,
    scrollbar_margin: Option<u16>,
    scroller_width: Option<u16>,
) -> WrappedWidgetBuilder {
    ScrollableBuilder {
        state: Some(state.0.clone()),
        spacing,
        padding,
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        max_width,
        max_height,
        align_items: align_items.map(|o| o.0),
        scrollbar_width,
        scrollbar_margin,
        scroller_width,
    }
    .into()
}

impl ToNative for ScrollableBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        scrollable_with_state(self.state.as_ref(), |state| {
            let el = Scrollable::new(state);
            let el = assign!(
                el,
                self,
                spacing,
                padding,
                width,
                height,
                max_width,
                max_height,
                align_items,
                scrollbar_width,
                scrollbar_margin,
                scroller_width,
            );
            Ok(el)
        })
    }
}
