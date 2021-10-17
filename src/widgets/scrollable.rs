use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_scrollbar, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ScrollableBuilder {
    pub state: iced::scrollable::State,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<iced::Align>,
    pub scrollbar_width: Option<u16>,
    pub scrollbar_margin: Option<u16>,
    pub scroller_width: Option<u16>,
    // style: TODO,
}

#[pyfunction(name="scrollbar")]
fn make_scrollbar<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for ScrollableBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        // let el = iced::Scrollable::new(&mut self.state);
        // let el = assign!(
        //     el, self, spacing, padding, width, height, max_width, max_height, align_items,
        //     scrollbar_width, scrollbar_margin, scroller_width,
        // );
        // el.into()
    }
}
