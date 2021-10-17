use std::sync::Arc;

use parking_lot::Mutex;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedButtonState>()?;
    Ok(())
}

pub(crate) type ButtonState = Arc<Mutex<iced::button::State>>;

#[pyclass(name="ButtonState", module="pyiced.pyiced")]
#[derive(Debug, Default)]
pub(crate) struct WrappedButtonState(pub ButtonState);

#[pyproto]
impl PyObjectProtocol for WrappedButtonState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl WrappedButtonState {
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

make_with_state! {
    button_with_state(
        iced::Button<Message>,
        iced::Button<'this, Message>,
        iced::button::State
    );
}
