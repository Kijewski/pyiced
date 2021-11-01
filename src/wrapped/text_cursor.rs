use std::sync::Arc;

use iced::text_input::State;
use parking_lot::Mutex;
use pyo3::prelude::*;

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
/// * `iced_native::widget::text_input::cursor::Cursor <https://docs.rs/iced_native/0.4.0/iced_native/widget/text_input/cursor/struct.Cursor.html>`_
#[pyclass(name = "TextInputCursor", module = "pyiced")]
#[derive(Default)]
pub(crate) struct WrappedTextCursor(pub Arc<Mutex<State>>);

#[pymethods]
impl WrappedTextCursor {
    #[new]
    fn new(state: &WrappedTextInputState) -> Self {
        Self(state.0.clone())
    }

    // TODO: state
    // TODO: selection
}
