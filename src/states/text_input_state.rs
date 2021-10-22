use std::sync::Arc;

use iced::text_input::State;
use parking_lot::Mutex;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTextInputState>()?;
    Ok(())
}

pub(crate) type TextInputState = Arc<Mutex<State>>;

/// TODO
#[pyclass(name = "TextInputState", module = "pyiced")]
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
    /// TODO
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }

    // TODO: cursor

    /// TODO
    fn is_focused(&self) -> PyResult<bool> {
        match self.0.try_lock() {
            Some(guard) => Ok(guard.is_focused()),
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// TODO
    fn focus(&self) -> PyResult<()> {
        match self.0.try_lock() {
            Some(mut guard) => {
                guard.focus();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// TODO
    fn unfocus(&self) -> PyResult<()> {
        match self.0.try_lock() {
            Some(mut guard) => {
                guard.unfocus();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// TODO
    fn move_cursor_to_front(&self) -> PyResult<()> {
        match self.0.try_lock() {
            Some(mut guard) => {
                guard.move_cursor_to_front();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// TODO
    fn move_cursor_to_end(&self) -> PyResult<()> {
        match self.0.try_lock() {
            Some(mut guard) => {
                guard.move_cursor_to_end();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// TODO
    fn move_cursor_to(&self, position: usize) -> PyResult<()> {
        match self.0.try_lock() {
            Some(mut guard) => {
                guard.move_cursor_to(position);
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }
}

make_with_state! {
    text_input_with_state(
        iced::TextInput<Message>,
        iced::TextInput<'this, Message>,
        iced::text_input::State,
    );
}
