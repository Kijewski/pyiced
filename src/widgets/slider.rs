use iced::{Element, Length, Slider};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
use crate::states::{slider_with_state, SliderState, WrappedSliderState};
use crate::styles::{SliderStyles, WrappedSliderStyle};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedLength, WrappedMessage};

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
    pub on_release: Option<Message>,
    pub width: Option<Length>,
    pub height: Option<u16>,
    pub step: Option<f32>,
    pub style: Option<SliderStyles>,
}

impl GCProtocol for SliderBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_change)?;
        Ok(())
    }
}

#[pyfunction(name = "slider")]
/// slider($module, /, state, start, end, value, on_change, *, on_release=None, width=None, height=None, step=1.0, style=None, style_hoverer=None, style_dragging=None)
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
/// on_change : Callable[[float], Optional[Message]]
///     Function to call with the new value.
/// on_release : Optional[Message]
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
/// style : SliderStyle
///     The normal style of the slider.
/// style_hoverer : SliderStyle
///     The style of the slider while hovering.
/// style_dragging : SliderStyle
///     The style of the slider while dragging.
///
/// Returns
/// -------
/// Element
///     The newly created .
///
/// See also
/// --------
/// * `iced_native::widget::slider::Slider <https://docs.rs/iced_native/0.4.0/iced_native/widget/slider/struct.Slider.html>`_
fn make_slider(
    state: &WrappedSliderState,
    start: f32,
    end: f32,
    value: f32,
    on_change: Py<PyAny>,
    on_release: Option<&WrappedMessage>,
    width: Option<&WrappedLength>,
    height: Option<u16>,
    step: Option<f32>,
    style: Option<&WrappedSliderStyle>,
    style_hoverer: Option<&WrappedSliderStyle>,
    style_dragging: Option<&WrappedSliderStyle>,
) -> PyResult<WrappedWidgetBuilder> {
    if !start.is_finite()
        || !end.is_finite()
        || !value.is_finite()
        || !step.map_or(true, |o| o.is_finite())
    {
        return Err(PyErr::new::<PyValueError, _>(
            "The arguments start, end, value and step need to be finite.",
        ));
    }
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
        on_release: on_release.map(|o| o.0.clone()),
        width: width.map(|o| o.0),
        height,
        step,
        style: SliderStyles::new(style, style_hoverer, style_dragging),
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
                Some(on_release) => el.on_release(on_release.clone()),
                None => el,
            };
            Ok(el)
        })
    }
}
