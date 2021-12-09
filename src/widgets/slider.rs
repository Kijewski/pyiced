use iced::{Element, Length, Slider};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{some_err, validate_f32, GCProtocol, Message, ToNative};
use crate::states::{slider_with_state, SliderState, WrappedSliderState};
use crate::styles::{SliderStyleSheet, WrappedSliderStyleSheet};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_slider, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SliderBuilder {
    pub token: Py<PyAny>,
    pub state: SliderState,
    pub start: f32,
    pub end: f32,
    pub value: f32,
    pub width: Option<Length>,
    pub height: Option<u16>,
    pub step: Option<f32>,
    pub style: Option<SliderStyleSheet>,
}

impl GCProtocol for SliderBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.token)?;
        Ok(())
    }
}

#[pyfunction(name = "slider")]
/// slider($module, /, token, state, start, end, value, step=1.0, *, width=None, height=None, style=None)
/// --
///
/// An horizontal bar and a handle that selects a single value from a range of values.
///
/// Parameters
/// ----------
/// token : object
///     When the user select a value, a message ``(token, new_value)`` is sent to the app's :meth:`~pyiced.IcedApp.update()` method.
///
///     When the user releases the pressed slider ``(token, None, 'release')`` is sent.
/// state : SliderState
///     Current state of the slider. The same object must be given between calls.
/// start : float
///     Smallest value inside the range.
/// end : float
///     Biggest value inside the range.
/// value : float
///     Current value.
/// step : float
///     Step size of the slider.
/// width : Optional[Length]
///     Width of the slider.
/// height : Optional[int]
///     Height of the slider.
/// style : SliderStyleSheet
///     The normal style of the slider.
///
/// Returns
/// -------
/// Element
///     The newly created slider.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/slider.png
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/slider.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::slider::Slider <https://docs.rs/iced_native/0.4.0/iced_native/widget/slider/struct.Slider.html>`_
fn make_slider(
    token: Py<PyAny>,
    state: &WrappedSliderState,
    start: f32,
    end: f32,
    value: f32,
    step: Option<f32>,
    width: Option<&WrappedLength>,
    height: Option<u16>,
    style: Option<&WrappedSliderStyleSheet>,
) -> PyResult<WrappedWidgetBuilder> {
    let start = validate_f32(start)?;
    let end = validate_f32(end)?;
    let value = validate_f32(value)?;

    if start > end || start > value || value > end {
        return Err(PyErr::new::<PyValueError, _>(
            "The following comparison must be true: start <= value <= end",
        ));
    }

    let el = SliderBuilder {
        token,
        state: state.0.clone(),
        start,
        end,
        value,
        width: width.map(|o| o.0),
        height,
        step: some_err(step.map(validate_f32))?,
        style: style.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for SliderBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let token = self.token.clone();
        let on_release = Message::Python((&token, (), "release").into_py(py));
        let on_change =
            move |value| Python::with_gil(|py| Message::Python((token.clone(), value).into_py(py)));
        slider_with_state(&self.state, |state| {
            let range = self.start..=self.end;
            let el = Slider::new(state, range, self.value, on_change);
            let el = assign!(el, self, width, height, step, style);
            let el = el.on_release(on_release.clone());
            Ok(el)
        })
    }
}
