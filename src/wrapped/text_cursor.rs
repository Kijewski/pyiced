use iced::text_input::State;
use parking_lot::lock_api::ArcMutexGuard;
use parking_lot::RawMutex;
use pyo3::prelude::*;
use pyo3::{PyGCProtocol, PyTraverseError, PyVisit};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTextCursor>()?;
    Ok(())
}

/// TextCursor
/// --
#[pyclass(name = "TextCursor", module = "pyiced", unsendable)]
#[derive(Default)]
pub(crate) struct WrappedTextCursor(pub Option<ArcMutexGuard<RawMutex, State>>);

#[pyproto]
impl PyGCProtocol for WrappedTextCursor {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        *self = Self::default();
    }
}

#[pymethods]
impl WrappedTextCursor {
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __exit__(
        &mut self,
        exc_type: Option<&PyAny>,
        exc_value: Option<&PyAny>,
        traceback: Option<&PyAny>,
    ) {
        let _ = exc_type;
        let _ = exc_value;
        let _ = traceback;
        self.0 = None;
    }
}

impl WrappedTextCursor {
    // TODO: state
    // TODO: selection
}
