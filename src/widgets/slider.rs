use std::ops::RangeInclusive;

use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_slider, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SliderBuilder {
    pub state: iced::slider::State,
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
fn make_slider<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for SliderBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        // let on_change = to_msg_fn(&self.on_change.unwrap());
        // let el = iced::Slider::new(&mut self.state, self.range.clone(), self.value, on_change);
        // let el = assign!(el, self, width, height, step);
        // let el = match &self.on_release {
        //     Some(on_release) => el.on_release(on_release.clone()),
        //     _ => el,
        // };
        // el.into()
    }
}
