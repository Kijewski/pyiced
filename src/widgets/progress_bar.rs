use iced::{Element, Length, ProgressBar};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_progress_bar, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProgressBarBuilder {
    pub start: f32,
    pub end: f32,
    pub value: f32,
    pub width: Option<Length>,
    pub height: Option<Length>,
    // style: TODO,
}

impl GCProtocol for ProgressBarBuilder {}

#[pyfunction(name = "progress_bar")]
fn make_progress_bar(
    start: f32,
    end: f32,
    value: f32,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    ProgressBarBuilder {
        start,
        end,
        value,
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
    }
    .into()
}

impl ToNative for ProgressBarBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let range = self.start..=self.end;
        let el = ProgressBar::new(range, self.value);
        let el = assign!(el, self, width, height);
        el.into()
    }
}
