use std::sync::Arc;

use iced::text_input::State;
use parking_lot::Mutex;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTextInputState>()?;
    Ok(())
}

pub(crate) type TextInputState = Arc<Mutex<State>>;

#[pyclass(name="TextInputState", module="pyiced.pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedTextInputState(pub TextInputState);

#[pyproto]
impl PyObjectProtocol for WrappedTextInputState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl WrappedTextInputState {
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

make_with_state! {
    text_input_with_state(
        iced::TextInput<Message>,
        iced::TextInput<'this, Message>,
        iced::text_input::State,
    );
}
