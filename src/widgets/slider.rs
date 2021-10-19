use iced::{Element, Length, Slider};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{empty_space, to_msg_fn, GCProtocol, Message, NonOptional, ToNative};
use crate::states::{slider_with_state, SliderState, WrappedSliderState};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedLength, WrappedMessage};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_slider, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SliderBuilder {
    pub state: NonOptional<SliderState>,
    pub start: f32,
    pub end: f32,
    pub value: f32,
    pub on_change: NonOptional<Py<PyAny>>, // fn f(value: Float) -> crate::Message
    pub on_release: Option<Message>,
    pub width: Option<Length>,
    pub height: Option<u16>,
    pub step: Option<f32>,
    // style: TODO,
}

impl GCProtocol for SliderBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        if let Some(on_change) = &self.on_change {
            visit.call(on_change)?;
        }
        Ok(())
    }
}

#[pyfunction(name = "slider")]
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
        state: Some(state.0.clone()),
        start,
        end,
        value,
        on_change: Some(on_change),
        on_release: on_release.map(|o| o.0.clone()),
        width: width.map(|o| o.0),
        height,
        step,
    }
    .into()
}

impl ToNative for SliderBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let on_change= match &self.on_change{
            Some(on_change) => on_change,
            None => return empty_space(),
        };
        slider_with_state(self.state.as_ref(), |state| {
            let on_change = to_msg_fn(on_change);
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
