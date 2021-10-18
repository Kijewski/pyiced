use std::ops::RangeInclusive;

use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, NonOptional, ToNative, to_msg_fn};
use crate::states::{SliderState, WrappedSliderState, slider_with_state};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedLength, WrappedMessage};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_slider, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SliderBuilder {
    pub state: NonOptional<SliderState>,
    pub range: RangeInclusive<f32>,
    pub value: f32,
    pub on_change: NonOptional<Py<PyAny>>, // fn f(value: Float) -> crate::Message
    pub on_release: Option<Message>,
    pub width: Option<iced::Length>,
    pub height: Option<u16>,
    pub step: Option<f32>,
    // style: TODO,
}

#[pyfunction(name="slider")]
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
        range: start..=end,
        value,
        on_change: Some(on_change),
        on_release: on_release.map(|o| o.0.clone()),
        width: width.map(|o| o.0.clone()),
        height,
        step,
    }.into()
}

impl ToNative for SliderBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        slider_with_state(self.state.as_ref(), |state| {
            let on_change = to_msg_fn(self.on_change.as_ref().unwrap());
            let el = iced::Slider::new(state, self.range.clone(), self.value, on_change);
            let el = assign!(el, self, width, height, step);
            let el = match &self.on_release {
                Some(on_release) => el.on_release(on_release.clone()),
                None => el,
            };
            Ok(el)
        })
    }
}
