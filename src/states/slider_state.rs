use std::sync::Arc;

use iced::slider::State;
use parking_lot::RwLock;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderState>()?;
    Ok(())
}

pub(crate) type SliderState = Arc<RwLock<State>>;

/// SliderState()
/// --
///
/// The state of a :func:`~pyiced.slider()`.
#[pyclass(name = "SliderState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedSliderState(pub SliderState);

#[pymethods]
impl WrappedSliderState {
    /// TODO
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

make_with_state! {
    slider_with_state(
        iced::Slider<f32, Message>,
        iced::Slider<'this, f32, Message>,
        iced::slider::State,
    );
}
