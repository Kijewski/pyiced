use iced::{Align, Element, Length, Scrollable};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::states::{scrollable_with_state, ScrollableState, WrappedScrollableState};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedAlign, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_scrollable, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ScrollableBuilder {
    pub state: ScrollableState,
    pub children: Vec<WidgetBuilder>,
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
    // pub on_scroll: Py<PyAny>, // fn f(value: f32) -> crate::Message
    // style: TODO,
}

impl GCProtocol for ScrollableBuilder {}

#[pyfunction(name = "scrollable")]
fn make_scrollable(
    py: Python,
    state: &WrappedScrollableState,
    children: &PyAny,
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
    // on_scroll: Py<PyAny>,
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
    let el = ScrollableBuilder {
        state: state.0.clone(),
        children,
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
        // on_scroll,
    };
    Ok(el.into())
}

impl ToNative for ScrollableBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        scrollable_with_state(&self.state, |state| {
            let el = Scrollable::new(state);
            // let el = match self.on_scroll.is_none(py) {
            //     false => el.on_scroll(to_msg_fn(&self.on_scroll)),
            //     true => el,
            // };
            let el = self.children.iter().fold(el, |el, c| el.push(c.to_native(py)));
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
