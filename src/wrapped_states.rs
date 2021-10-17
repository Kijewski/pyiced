use std::sync::Arc;

use parking_lot::Mutex;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::debug_str;

pub(crate) fn init(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ButtonState>()?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub(crate) struct ButtonState(pub Arc<Mutex<iced::button::State>>);

#[pyproto(module="pyiced.pyiced")]
impl PyObjectProtocol for ButtonState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl ButtonState {
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}
