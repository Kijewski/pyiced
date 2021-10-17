use iced::{Element, Length, Slider};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, validate_f32, GCProtocol, Message, ToNative};
use crate::states::{slider_with_state, SliderState, WrappedSliderState};
use crate::styles::{SliderStyleSheet, WrappedSliderStyleSheet};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{MessageOrDatum, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_slider, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SliderBuilder {
    pub state: SliderState,
    pub start: f32,
    pub end: f32,
    pub value: f32,
    pub on_change: Py<PyAny>, // fn f(value: Float) -> crate::Message
    pub on_release: Message,
    pub width: Option<Length>,
    pub height: Option<u16>,
    pub step: Option<f32>,
    pub style: Option<SliderStyleSheet>,
}

impl GCProtocol for SliderBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_change)?;
        self.on_release.traverse(visit)?;
        Ok(())
    }
}

#[pyfunction(name = "slider")]
/// slider($module, /, state, start, end, value, on_change, *, on_release=None, width=None, height=None, step=1.0, style=None)
/// --
///
/// An horizontal bar and a handle that selects a single value from a range of values.
///
/// Parameters
/// ----------
/// state : SliderState
///     Current state of the slider. The same object must be given between calls.
/// start : float
///     Smallest value inside the range.
/// end : float
///     Biggest value inside the range.
/// value : float
///     Current value.
/// on_change : Callable[[float], Optional[object]]
///     Function to call with the new value.
///
///     The function can return a message that will be received in the app's :meth:`~pyiced.IcedApp.update` loop.
/// on_release : Optional[object]
///     Sets the release message of the Slider. This is called when the mouse is released from the slider.
///
///     Typically, the user’s interaction with the slider is finished when this message is produced.
///     This is useful if you need to spawn a long-running task from the slider’s result, where the default on_change message could create too many events.
/// width : Optional[Length]
///     Width of the slider.
/// height : Optional[int]
///     Height of the slider.
/// step : float
///     Step size of the slider.
/// style : SliderStyleSheet
///     The normal style of the slider.
///
/// Returns
/// -------
/// Element
///     The newly created .
///
/// See also
/// --------
/// `iced_native::widget::slider::Slider <https://docs.rs/iced_native/0.4.0/iced_native/widget/slider/struct.Slider.html>`_
fn make_slider(
    state: &WrappedSliderState,
    start: f32,
    end: f32,
    value: f32,
    on_change: Py<PyAny>,
    on_release: Option<MessageOrDatum>,
    width: Option<&WrappedLength>,
    height: Option<u16>,
    step: Option<f32>,
    style: Option<&WrappedSliderStyleSheet>,
) -> PyResult<WrappedWidgetBuilder> {
    let start = validate_f32(start)?;
    let end = validate_f32(end)?;
    let value = validate_f32(value)?;
    let step = match step {
        Some(step) => Some(validate_f32(step)?),
        None => None,
    };

    if start > end || start > value || value > end {
        return Err(PyErr::new::<PyValueError, _>(
            "The following comparison must be true: start <= value <= end",
        ));
    }

    let el = SliderBuilder {
        state: state.0.clone(),
        start,
        end,
        value,
        on_change,
        on_release: on_release.unwrap_or_default().into(),
        width: width.map(|o| o.0),
        height,
        step,
        style: style.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for SliderBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let on_change = to_msg_fn(&self.on_change);
        slider_with_state(&self.state, |state| {
            let range = self.start..=self.end;
            let el = Slider::new(state, range, self.value, on_change);
            let el = assign!(el, self, width, height, step, style);
            let el = match &self.on_release {
                Message::None => el,
                on_release => el.on_release(on_release.clone()),
            };
            Ok(el)
        })
    }
}
