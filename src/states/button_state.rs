use std::sync::Arc;

use iced::button::State;
use parking_lot::RwLock;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedButtonState>()?;
    Ok(())
}

pub(crate) type ButtonState = Arc<RwLock<State>>;

/// ButtonState()
/// --
///
/// The state of a :func:`~pyiced.button()`.
#[pyclass(name = "ButtonState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedButtonState(pub ButtonState);

#[pymethods]
impl WrappedButtonState {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

make_with_state! {
    button_with_state(
        iced::Button<Message>,
        iced::Button<'this, Message>,
        iced::button::State
    );
}
