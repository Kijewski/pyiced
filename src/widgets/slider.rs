use iced::{Element, Length, Slider};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
use crate::states::{slider_with_state, SliderState, WrappedSliderState};
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
    // style: TODO,
}

impl GCProtocol for SliderBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_change)?;
        Ok(())
    }
}

#[pyfunction(name = "slider")]
/// slider($module, /, state, start, end, value, on_change, *, on_release=None, width=None, height=None, step=None)
/// --
///
/// Make a .
///
/// Parameters
/// ----------
/// state : SliderState
///     TODO
/// start : float
///     TODO
/// end : float
///     TODO
/// value : float
///     TODO
/// on_change : Callable[[float], Optional[Message]]
///     TODO
/// on_release : Optional[Message]
///     TODO
/// width : Optional[Length]
///     TODO
/// height : Optional[int]
///     TODO
/// step : Optiona[float]
///     TODO
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
) -> WrappedWidgetBuilder {
    SliderBuilder {
        state: state.0.clone(),
        start,
        end,
        value,
        on_change,
        on_release: on_release.map(|o| o.0.clone()),
        width: width.map(|o| o.0),
        height,
        step,
    }
    .into()
}

impl ToNative for SliderBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let on_change = to_msg_fn(&self.on_change);
        slider_with_state(&self.state, |state| {
            let range = self.start..=self.end;
            let el = Slider::new(state, range, self.value, on_change);
            let el = assign!(el, self, width, height, step);
            let el = match &self.on_release {
                Some(on_release) => el.on_release(on_release.clone()),
                None => el,
            };
            Ok(el)
        })
    }
}
