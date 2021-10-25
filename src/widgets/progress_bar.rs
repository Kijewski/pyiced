use iced::{Element, Length, ProgressBar};
use pyo3::exceptions::PyValueError;
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
/// progress_bar($module, /, start, end, value, *, width=None, height=None)
/// --
///
/// A bar that displays progress.
///
/// Parameters
/// ----------
/// start : f32
///     Minimum value inside the value range.
/// end : f32
///     Maximum value inside the value range.
/// value : f32
///     Current value of the progress bar.
/// width : Optional[Length]
///     Width of the progress bar.
/// height : Optional[Length]
///     Height of the progress bar.
///
/// Returns
/// -------
/// Element
///     The newly created progress bar.
///
/// See also
/// --------
/// * `iced_native::widget::progress_bar::ProgressBar <https://docs.rs/iced_native/0.4.0/iced_native/widget/progress_bar/struct.ProgressBar.html>`_
fn make_progress_bar(
    start: f32,
    end: f32,
    value: f32,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> PyResult<WrappedWidgetBuilder> {
    if !start.is_finite() || !end.is_finite() || !value.is_finite() {
        return Err(PyErr::new::<PyValueError, _>(
            "The arguments start, end and value need to be finite.",
        ));
    }
    if start > end || start > value || value > end {
        return Err(PyErr::new::<PyValueError, _>(
            "The following comparison must be true: start <= value <= end",
        ));
    }
    let el = ProgressBarBuilder {
        start,
        end,
        value,
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for ProgressBarBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let range = self.start..=self.end;
        let el = ProgressBar::new(range, self.value);
        let el = assign!(el, self, width, height);
        el.into()
    }
}
