use iced::{Element, Length, ProgressBar};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{validate_f32, GCProtocol, Message, ToNative};
use crate::styles::{ProgressBarStyle, WrappedProgressBarStyle};
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
    pub style: Option<ProgressBarStyle>,
}

impl GCProtocol for ProgressBarBuilder {}

#[pyfunction(name = "progress_bar")]
/// progress_bar($module, /, start, end, value, *, width=None, height=None, style=None)
/// --
///
/// A bar that displays progress.
///
/// Parameters
/// ----------
/// start : float
///     Minimum value inside the value range.
/// end : float
///     Maximum value inside the value range.
/// value : float
///     Current value of the progress bar.
/// width : Optional[Length]
///     Width of the progress bar.
/// height : Optional[Length]
///     Height of the progress bar.
/// style : Optional[ProgressBarStyleSheet]
///     Style of the progress bar.
///
/// Returns
/// -------
/// Element
///     The newly created progress bar.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/progress_bar.png
///    :width: 688
///    :height: 405
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/progress_bar.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::progress_bar::ProgressBar <https://docs.rs/iced_native/0.4.0/iced_native/widget/progress_bar/struct.ProgressBar.html>`_
fn make_progress_bar(
    start: f32,
    end: f32,
    value: f32,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    style: Option<&WrappedProgressBarStyle>,
) -> PyResult<WrappedWidgetBuilder> {
    let start = validate_f32(start)?;
    let end = validate_f32(end)?;
    let value = validate_f32(value)?;
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
        style: style.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for ProgressBarBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let range = self.start..=self.end;
        let el = ProgressBar::new(range, self.value);
        let el = assign!(el, self, width, height, style);
        el.into()
    }
}
