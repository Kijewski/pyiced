use std::sync::Arc;

use iced::text_input::State;
use iced_native::text_input::Value;
use parking_lot::RwLock;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::common::EitherPy;
use crate::states::WrappedTextInputState;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTextCursor>()?;
    Ok(())
}

/// TextInputCursor(state)
/// --
///
/// A representation of cursor position in a :func:`~pyiced.text_input()`.
///
/// There should be no reason to create or inspect this object directly.
///
/// Parameters
/// ----------
/// state : TextInputState
///     Text input state to inspect.
///
/// See also
/// --------
/// `iced_native::widget::text_input::cursor::Cursor <https://docs.rs/iced_native/0.4.0/iced_native/widget/text_input/cursor/struct.Cursor.html>`_
#[pyclass(name = "TextInputCursor", module = "pyiced")]
#[derive(Default)]
pub(crate) struct WrappedTextCursor(pub Arc<RwLock<State>>);

#[pymethods]
impl WrappedTextCursor {
    #[new]
    fn new(state: &WrappedTextInputState) -> Self {
        Self(state.0.clone())
    }

    /// state($self, /, value)
    /// --
    ///
    /// Get the state of the :func:`~pyiced.TextInputCursor`.
    ///
    /// The result is measured in terms of graphems, not bytes or codepoints!
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    ///
    /// See also
    /// --------
    /// :meth:`pyiced.TextInputState.move_cursor_to()`
    ///
    /// Returns
    /// -------
    /// int
    ///     The current cursor position when there's no selection.
    /// Tuple[int, int]
    ///     The selected text range.
    fn state(&self, value: &str) -> PyResult<EitherPy<usize, (usize, usize)>> {
        use iced_native::text_input::cursor::State::*;
        let cursor = match self.0.try_read() {
            Some(guard) => guard.cursor(),
            None => return Err(PyRuntimeError::new_err("State is in use")),
        };
        match cursor.state(&Value::new(value)) {
            Index(i) => Ok(EitherPy::Left(i)),
            Selection { start, end } => Ok(EitherPy::Right((start, end))),
        }
    }

    /// selection($self, /, value)
    /// --
    ///
    /// Get the selected text.
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    ///
    /// Parameters
    /// ----------
    /// value : str
    ///     The current value of the :func:`~pyiced.text_input()`.
    ///
    /// Returns
    /// -------
    /// str
    ///     The selected text. May be empty.
    fn selection(&self, value: &str) -> PyResult<String> {
        let cursor = match self.0.try_read() {
            Some(guard) => guard.cursor(),
            None => return Err(PyRuntimeError::new_err("State is in use")),
        };

        let value = Value::new(value);
        Ok(match cursor.selection(&value) {
            Some((start, end)) => value.select(start, end).to_string(),
            None => "".to_string(),
        })
    }
}
