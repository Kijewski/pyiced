use std::sync::Arc;

use parking_lot::Mutex;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderState>()?;
    Ok(())
}

pub(crate) type SliderState = Arc<Mutex<iced::slider::State>>;

#[pyclass(name="SliderState", module="pyiced.pyiced")]
#[derive(Debug, Default)]
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
