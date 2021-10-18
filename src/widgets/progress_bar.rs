use std::ops::RangeInclusive;

use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_progress_bar, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ProgressBarBuilder {
    pub range: RangeInclusive<f32>,
    pub value: f32,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    // style: TODO,
}

impl GCProtocol for ProgressBarBuilder {}

#[pyfunction(name="progress_bar")]
fn make_progress_bar<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for ProgressBarBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::ProgressBar::new(self.range.clone(), self.value);
        let el = assign!(el, self, width, height);
        el.into()
    }
}
