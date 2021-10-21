use std::sync::Arc;

use iced::slider::State;
use parking_lot::Mutex;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderState>()?;
    Ok(())
}

pub(crate) type SliderState = Arc<Mutex<State>>;

#[pyclass(name = "SliderState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedSliderState(pub SliderState);

#[pyproto]
impl PyObjectProtocol for WrappedSliderState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl WrappedSliderState {
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

make_with_state! {
    slider_with_state(
        iced::Slider<f32, Message>,
        iced::Slider<'this, f32, Message>,
        iced::slider::State,
    );
}
