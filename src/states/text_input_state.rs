use std::sync::Arc;

use iced::text_input::State;
use iced_native::text_input::Value;
use parking_lot::RwLock;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::common::{debug_str, EitherPy};
use crate::make_with_state;
use crate::wrapped::WrappedTextCursor;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTextInputState>()?;
    Ok(())
}

pub(crate) type TextInputState = Arc<RwLock<State>>;

/// TextInputState()
/// --
///
/// The state of a :func:`~pyiced.text_input()`.
///
/// See also
/// --------
/// `iced_native::widget::text_input::State <https://docs.rs/iced_native/0.4.0/iced_native/widget/text_input/struct.State.html>`_
#[pyclass(name = "TextInputState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedTextInputState(pub TextInputState);

#[pymethods]
impl WrappedTextInputState {
    /// TODO
    #[new]
    fn new() -> Self {
        Self::default()
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
            None => return Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        };

        let value = Value::new(value);
        Ok(match cursor.selection(&value) {
            Some((start, end)) => value.select(start, end).to_string(),
            None => "".to_string(),
        })
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
        let cursor = match self.0.try_read() {
            Some(guard) => guard.cursor(),
            None => return Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        };

        match cursor.state(&Value::new(value)) {
            iced_native::text_input::cursor::State::Index(index) => Ok(EitherPy::Left(index)),
            iced_native::text_input::cursor::State::Selection { start, end } => {
                Ok(EitherPy::Right((start, end)))
            },
        }
    }

    /// is_focused($self)
    /// --
    ///
    /// Returns whether the :func:`~pyiced.text_input()` is currently focused or not.
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    ///
    /// Returns
    /// -------
    /// bool
    ///     Yes or no
    fn is_focused(&self) -> PyResult<bool> {
        match self.0.try_read() {
            Some(guard) => Ok(guard.is_focused()),
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// focus($self)
    /// --
    ///
    /// Focuses the :func:`~pyiced.text_input()`.
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    fn focus(&self) -> PyResult<()> {
        match self.0.try_write() {
            Some(mut guard) => {
                guard.focus();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// unfocus($self)
    /// --
    ///
    /// Unfocuses the :func:`~pyiced.text_input()`.
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    fn unfocus(&self) -> PyResult<()> {
        match self.0.try_write() {
            Some(mut guard) => {
                guard.unfocus();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// move_cursor_to_front($self)
    /// --
    ///
    /// Moves the :func:`~pyiced.TextInputCursor` of the :class:`~pyiced.TextInput` to the front of the input text.
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    fn move_cursor_to_front(&self) -> PyResult<()> {
        match self.0.try_write() {
            Some(mut guard) => {
                guard.move_cursor_to_front();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// move_cursor_to_end($self)
    /// --
    ///
    /// Moves the :func:`~pyiced.TextInputCursor` of the :class:`~pyiced.TextInput` to the end of the input text.
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    fn move_cursor_to_end(&self) -> PyResult<()> {
        match self.0.try_write() {
            Some(mut guard) => {
                guard.move_cursor_to_end();
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// move_cursor_to($self, /, position)
    /// --
    ///
    /// Moves the :func:`~pyiced.TextInputCursor` of the :class:`~pyiced.TextInput` to an arbitrary location.
    ///
    /// The result is measured in terms of graphems, not bytes or codepoints!
    ///
    /// See also
    /// --------
    /// :meth:`pyiced.TextInputState.state()`
    ///
    /// Warning
    /// -------
    /// If the state is currently in use, the method will fail.
    ///
    /// Parameters
    /// ----------
    /// position : int
    ///     The new cursor position.
    fn move_cursor_to(&self, position: usize) -> PyResult<()> {
        match self.0.try_write() {
            Some(mut guard) => {
                guard.move_cursor_to(position);
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

make_with_state! {
    text_input_with_state(
        iced::TextInput<Message>,
        iced::TextInput<'this, Message>,
        iced::text_input::State,
    );
}
